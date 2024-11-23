extern crate rand;

use std::rc::Rc;

use rand::Rng;

use crate::{
    expression::{DeBruijnIndex, Expression, ExpressionArena, ExpressionId},
    referencing_environment::ReferencingEnvironment,
    strings::{StringArena, StringId},
};

impl Expression {
    pub fn sample<'a, R: Rng>(
        strings: &'a mut StringArena,
        expressions: &'a mut ExpressionArena,
        environment: Rc<ReferencingEnvironment>,
        rng: &'a mut R,
        max_depth: usize,
    ) -> ExpressionId {
        ExpressionSampler::new(
            strings,
            expressions,
            ReferencingEnvironment::new_frame(environment),
            rng,
        )
        .sample(max_depth)
    }
}

struct ExpressionSampler<'a, R: Rng> {
    strings: &'a mut StringArena,
    expressions: &'a mut ExpressionArena,
    environment: ReferencingEnvironment,
    rng: &'a mut R,
}

impl<'a, R: Rng> ExpressionSampler<'a, R> {
    pub fn new(
        strings: &'a mut StringArena,
        expressions: &'a mut ExpressionArena,
        environment: ReferencingEnvironment,
        rng: &'a mut R,
    ) -> ExpressionSampler<'a, R> {
        ExpressionSampler {
            strings,
            expressions,
            environment,
            rng,
        }
    }

    fn sample_expression(&mut self, max_depth: usize) -> ExpressionId {
        if max_depth == 0 {
            return self.sample_variable_expression();
        }

        match self.rng.gen_range(0..=7) {
            0 => self.sample_variable_expression(),
            1..=3 => self.sample_lambda_expression(max_depth),
            4..=7 => self.sample_application_expression(max_depth),
            _ => unreachable!(),
        }
    }

    fn sample_variable_expression(&mut self) -> ExpressionId {
        if self.environment.is_empty() {
            return self.sample_possibly_free_variable_expression();
        }

        if self.environment.domain_len() == 0 {
            return match self.rng.gen_range(0..=1) {
                0 => self.sample_possibly_free_variable_expression(),
                1 => self.sample_bound_nameless_variable_expression(),
                _ => unreachable!(),
            };
        }

        match self.rng.gen_range(0..=2) {
            0 => self.sample_possibly_free_variable_expression(),
            1 => self.sample_bound_nameless_variable_expression(),
            2 => self.sample_bound_variable_expression(),
            _ => unreachable!(),
        }
    }

    fn sample_possibly_free_variable_expression(&mut self) -> ExpressionId {
        let identifier = self.sample_identifier();
        self.expressions.variable(identifier)
    }

    fn sample_bound_variable_expression(&mut self) -> ExpressionId {
        let identifier = self.sample_bound_identifier();
        self.expressions.variable(identifier)
    }

    fn sample_bound_nameless_variable_expression(&mut self) -> ExpressionId {
        let index = self.sample_index();
        self.expressions.nameless_variable(index)
    }

    fn sample_lambda_expression(&mut self, max_depth: usize) -> ExpressionId {
        debug_assert!(max_depth > 0);
        match self.rng.gen_range(0..=1) {
            0 => self.sample_named_lambda_expression(max_depth),
            1 => self.sample_nameless_lambda_expression(max_depth),
            _ => unreachable!(),
        }
    }

    fn sample_named_lambda_expression(&mut self, max_depth: usize) -> ExpressionId {
        debug_assert!(max_depth > 0);
        let parameter = {
            if self.rng.gen_bool(0.2) {
                Option::None
            } else {
                let identifier = self.sample_identifier();
                Option::Some(identifier)
            }
        };
        self.environment.bind_option(parameter);
        let body = self.sample_expression(max_depth - 1);
        self.environment.unbind_option(parameter);
        self.expressions.abstraction(parameter, body)
    }

    fn sample_nameless_lambda_expression(&mut self, max_depth: usize) -> ExpressionId {
        debug_assert!(max_depth > 0);
        self.environment.shift();
        let body = self.sample_expression(max_depth - 1);
        self.environment.unshift();
        self.expressions.nameless_abstraction(body)
    }

    fn sample_application_expression(&mut self, max_depth: usize) -> ExpressionId {
        debug_assert!(max_depth > 0);
        let function = self.sample_expression(max_depth - 1);
        let arguments_count = self.rng.gen_range(1..=5);
        let mut arguments = Vec::with_capacity(arguments_count);
        for _ in 0..arguments_count {
            let argument = self.sample_expression(max_depth - 1);
            arguments.push(argument);
        }
        self.expressions.application(function, arguments)
    }

    fn sample_index(&mut self) -> DeBruijnIndex {
        debug_assert!(!self.environment.is_empty());
        let size = self.environment.len();
        self.rng.gen_range(1..=size).into()
    }

    fn sample_bound_identifier(&mut self) -> StringId {
        debug_assert!(self.environment.domain_len() > 0);
        let domain = self.environment.domain();
        let domain_list: Vec<StringId> = domain.iter().cloned().collect();
        domain_list[self.rng.gen_range(0..domain_list.len())]
    }

    fn sample_identifier(&mut self) -> StringId {
        let len = self.rng.gen_range(1..=5) as usize;
        let mut identifier = Vec::with_capacity(len);
        identifier.push(self.sample_alpha());
        for _ in 1..len {
            identifier.push(self.sample_alphanumeric());
        }
        self.strings.intern(&identifier)
    }

    fn sample_alpha(&mut self) -> u8 {
        if self.rng.gen_bool(0.5) {
            self.rng.gen_range(b'a'..=b'z')
        } else {
            self.rng.gen_range(b'A'..=b'Z')
        }
    }

    fn sample_alphanumeric(&mut self) -> u8 {
        if self.rng.gen_ratio(26 * 2, 26 * 2 + 10) {
            self.sample_alpha()
        } else {
            self.rng.gen_range(b'0'..=b'9')
        }
    }

    pub fn sample(mut self, max_depth: usize) -> ExpressionId {
        self.sample_expression(max_depth)
    }
}
