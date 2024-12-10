import { Code, LezerHighlighter, makeScene2D } from "@motion-canvas/2d";
import { parser } from "@lezer/rust";
import {
  all,
  beginSlide,
  createRef,
  Direction,
  slideTransition,
} from "@motion-canvas/core";

Code.defaultHighlighter = new LezerHighlighter(parser);

export default makeScene2D(function* (view) {
  const code = createRef<Code>();

  view.add(
    <Code
      ref={code}
      fontSize={34}
      width={1920}
      padding={100}
      code={`\
struct NameGeneration {

}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a> {
    strings: &'a mut StringArena
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a> {
    strings: &'a mut StringArena,
    source: &'a ExpressionArena
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a> {
    strings: &'a mut StringArena,
    source: &'a ExpressionArena,
    destination: &'a mut ExpressionArena
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a> {
    strings: &'a mut StringArena,
    source: &'a ExpressionArena,
    destination: &'a mut ExpressionArena,
    identifiers: IdentifierArena
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a> {
    strings: &'a mut StringArena,
    source: &'a ExpressionArena,
    destination: &'a mut ExpressionArena,
    identifiers: IdentifierArena,
    constraints: ConstraintStore
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a> {
    strings: &'a mut StringArena,
    source: &'a ExpressionArena,
    destination: &'a mut ExpressionArena,
    identifiers: IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> {
    strings: &'a mut StringArena,
    source: &'a ExpressionArena,
    destination: &'a mut ExpressionArena,
    identifiers: IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
    variable_name_generator: G
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* all(
    code().fontSize(30, 1),
    code().code(
      `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {

}`,
      1
    )
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn lookup_restriction_set(&self, restrictions: &HashSet<IdentifierId>) -> HashSet<StringId> {
        
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn lookup_restriction_set(&self, restrictions: &HashSet<IdentifierId>) -> HashSet<StringId> {
        let mut identifiers = HashSet::new();
        
        identifiers
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn lookup_restriction_set(&self, restrictions: &HashSet<IdentifierId>) -> HashSet<StringId> {
        let mut identifiers = HashSet::new();
        for restriction in restrictions {
            
        }
        identifiers
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn lookup_restriction_set(&self, restrictions: &HashSet<IdentifierId>) -> HashSet<StringId> {
        let mut identifiers = HashSet::new();
        for restriction in restrictions {
            if let Option::Some(identifier) = self.identifiers.lookup(*restriction) {
                identifiers.insert(identifier);
            }
        }
        identifiers
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {

        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {

            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {
                let Constraint {
                    parameter,
                    restrictions,
                    used,
                } = self.constraints.get(expression).unwrap();
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let chosen_parameter =
                    if let Option::Some(name) = initial_parameter {
                        
                    };
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let chosen_parameter =
                    if let Option::Some(name) = initial_parameter {
                        let claimed_identifiers = self.lookup_restriction_set(restrictions);
                        if claimed_identifiers.contains(name) {
                            
                        } else {
                         
                        }
                    };
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let chosen_parameter =
                    if let Option::Some(name) = initial_parameter {
                        let claimed_identifiers = self.lookup_restriction_set(restrictions);
                        if claimed_identifiers.contains(name) {
                            let new_name = self
                                .variable_name_generator
                                .fresh_name(self.strings, &claimed_identifiers);
                            self.identifiers.set(*parameter, new_name);
                            Option::Some(new_name)
                        } else {
                            
                        }
                    };
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let chosen_parameter =
                    if let Option::Some(name) = initial_parameter {
                        let claimed_identifiers = self.lookup_restriction_set(restrictions);
                        if claimed_identifiers.contains(name) {
                            let new_name = self
                                .variable_name_generator
                                .fresh_name(self.strings, &claimed_identifiers);
                            self.identifiers.set(*parameter, new_name);
                            Option::Some(new_name)
                        } else {
                            self.identifiers.set(*parameter, *name);
                            Option::Some(*name)
                        }
                    };
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let chosen_parameter =
                    if let Option::Some(name) = initial_parameter {
                        // ...
                    };
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let chosen_parameter =
                    if let Option::Some(name) = initial_parameter {
                        // ...
                    } else if *used {
                        let claimed_identifiers = self.lookup_restriction_set(restrictions);
                        let name = self
                            .variable_name_generator
                            .fresh_name(self.strings, &claimed_identifiers);
                        self.identifiers.set(*parameter, name);
                        Option::Some(name)
                    };
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let chosen_parameter =
                    if let Option::Some(name) = initial_parameter {
                        // ...
                    } else if *used {
                        let claimed_identifiers = self.lookup_restriction_set(restrictions);
                        let name = self
                            .variable_name_generator
                            .fresh_name(self.strings, &claimed_identifiers);
                        self.identifiers.set(*parameter, name);
                        Option::Some(name)
                    } else {
                        Option::None
                    };
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let chosen_parameter = /* ... */;
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let chosen_parameter = /* ... */;
                match initial_parameter {
                    Option::Some(name) => {
                    
                    }
                    Option::None => {

                    }
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let chosen_parameter = /* ... */;
                match initial_parameter {
                    Option::Some(name) => {
                        self.environment.bind(*name, *parameter, expression);
                        let named_body = self.convert_to_named(*body);
                        self.environment.unbind(*name);
                        self.destination.abstraction(chosen_parameter, named_body)
                    }
                    Option::None => {
                    
                    }
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: initial_parameter, body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let chosen_parameter = /* ... */;
                match initial_parameter {
                    Option::Some(name) => {
                        self.environment.bind(*name, *parameter, expression);
                        let named_body = self.convert_to_named(*body);
                        self.environment.unbind(*name);
                        self.destination.abstraction(chosen_parameter, named_body)
                    }
                    Option::None => {
                        self.environment.shift(expression);
                        let named_body = self.convert_to_named(*body);
                        self.environment.unshift();
                        self.destination.abstraction(chosen_parameter, named_body)
                    }
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessAbstraction { body } => {
                
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessAbstraction { body } => {
                let Constraint {
                    parameter,
                    restrictions,
                    used,
                } = self.constraints.get(expression).unwrap();
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessAbstraction { body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessAbstraction { body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let parameter = if *used {
                    let claimed_identifiers = self.lookup_restriction_set(restrictions);
                    let name = self
                        .variable_name_generator
                        .fresh_name(self.strings, &claimed_identifiers);
                    self.identifiers.set(*parameter, name);
                    Option::Some(name)
                } else {
                    Option::None
                };
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessAbstraction { body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let parameter = /* ... */;
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessAbstraction { body } => {
                let Constraint { parameter, restrictions, used } = /* ... */;
                let parameter = /* ... */;
                self.environment.shift(expression);
                let named_body = self.convert_to_named(*body);
                self.environment.unshift();
                self.destination.abstraction(parameter, named_body)
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {

        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Variable { identifier } => {
            
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Variable { identifier } => {
                match self.environment.lookup(*name) {
                    Option::Some(identifier) => {

                    }
                    Option::None => {
                    
                    }
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Variable { identifier } => {
                match self.environment.lookup(*name) {
                    Option::Some(identifier) => {
                        let assigned_name = self.identifiers.lookup(identifier).unwrap();
                        self.destination.variable(assigned_name)
                    }
                    Option::None => {
                        
                    }
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Variable { identifier } => {
                match self.environment.lookup(*name) {
                    Option::Some(identifier) => {
                        let assigned_name = self.identifiers.lookup(identifier).unwrap();
                        self.destination.variable(assigned_name)
                    }
                    Option::None => {
                        self.destination.variable(*name)
                    }
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessVariable { index } => {
                
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessVariable { index } => {
                let binder = self.environment.lookup_binder(*index);
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessVariable { index } => {
                let binder = self.environment.lookup_binder(*index);
                let Constraint {
                    parameter,
                    restrictions: _,
                    used: _,
                } = self.constraints.get(binder).unwrap();
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessVariable { index } => {
                let binder = self.environment.lookup_binder(*index);
                let Constraint {
                    parameter,
                    restrictions: _,
                    used: _,
                } = self.constraints.get(binder).unwrap();
                let identifier = self.identifiers.lookup(*parameter).unwrap();
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessVariable { index } => {
                let binder = self.environment.lookup_binder(*index);
                let Constraint {
                    parameter,
                    restrictions: _,
                    used: _,
                } = self.constraints.get(binder).unwrap();
                let identifier = self.identifiers.lookup(*parameter).unwrap();
                self.destination.variable(identifier)
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Application { function, arguments } => {

            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Application { function, arguments } => {
                let named_function = self.convert_to_named(*function);
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Application { function, arguments } => {
                let named_function = self.convert_to_named(*function);
                let mut named_arguments = Vec::with_capacity(arguments.len());
                for &argument in arguments {
                    let named_argument = self.convert_to_named(argument);
                    named_arguments.push(named_argument);
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: FreshVariableNameGenerator> { /* ... */ }

impl<'a, G: FreshVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Application { function, arguments } => {
                let named_function = self.convert_to_named(*function);
                let mut named_arguments = Vec::with_capacity(arguments.len());
                for &argument in arguments {
                    let named_argument = self.convert_to_named(argument);
                    named_arguments.push(named_argument);
                }
                self.destination.application(named_function, named_arguments)
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");
});
