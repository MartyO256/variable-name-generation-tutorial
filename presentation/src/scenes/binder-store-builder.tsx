import { Code, LezerHighlighter, makeScene2D } from "@motion-canvas/2d";
import { parser } from "@lezer/rust";
import {
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
      fontSize={30}
      width={1920}
      padding={100}
      code={`\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {

}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
        
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let binder = Binder::new(*parameter, parameter_identifier);
                self.binders.set(expression, binder);
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let binder = Binder::new(*parameter, parameter_identifier);
                self.binders.set(expression, binder);
                match parameter {
                    Option::Some(parameter) => {
                        
                    }
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let binder = Binder::new(*parameter, parameter_identifier);
                self.binders.set(expression, binder);
                match parameter {
                    Option::Some(parameter) => {
                        self.environment
                            .bind(*parameter, parameter_identifier, expression);
                        self.visit(*body);
                        self.environment.unbind(*parameter);
                    }
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let binder = Binder::new(*parameter, parameter_identifier);
                self.binders.set(expression, binder);
                match parameter {
                    
                }
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let binder = Binder::new(*parameter, parameter_identifier);
                self.binders.set(expression, binder);
                match parameter {
                    Option::None => {
                    
                    }
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let binder = Binder::new(*parameter, parameter_identifier);
                self.binders.set(expression, binder);
                match parameter {
                    Option::None => {
                        self.environment.shift(expression);
                        self.visit(*body);
                        self.environment.unshift();
                    }
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessAbstraction { body } => {
            
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessAbstraction { body } => {
                let parameter_identifier = self.identifiers.new_identifier();
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessAbstraction { body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let binder = Binder::new(Option::None, parameter_identifier);
                self.binders.set(expression, binder);
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
struct BinderStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
}

impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessAbstraction { body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let binder = Binder::new(Option::None, parameter_identifier);
                self.binders.set(expression, binder);
                self.environment.shift(expression);
                self.visit(*body);
                self.environment.unshift();
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
        
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
            
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                if let Option::Some(identifier) = self.environment.lookup(*variable) {
                
                } else {

                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                if let Option::Some(identifier) = self.environment.lookup(*variable) {
                    let mut undesirables = Vec::new();
                    for binder_expression in self.environment.binders_iter() {
                        
                    }
                } else {

                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                if let Option::Some(identifier) = self.environment.lookup(*variable) {
                    let mut undesirables = Vec::new();
                    for binder_expression in self.environment.binders_iter() {
                        let binder = self.binders.get_mut(*binder_expression).unwrap();
                        if binder.destination_parameter == identifier {
                            
                        }
                    }
                } else {

                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                if let Option::Some(identifier) = self.environment.lookup(*variable) {
                    let mut undesirables = Vec::new();
                    for binder_expression in self.environment.binders_iter() {
                        let binder = self.binders.get_mut(*binder_expression).unwrap();
                        if binder.destination_parameter == identifier {
                            for undesirable in undesirables {
                                binder.add_string_undesirable(undesirable);
                            }
                            binder.mark_used();
                            break;
                        }
                    }
                } else {

                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                if let Option::Some(identifier) = self.environment.lookup(*variable) {
                    let mut undesirables = Vec::new();
                    for binder_expression in self.environment.binders_iter() {
                        let binder = self.binders.get_mut(*binder_expression).unwrap();
                        if binder.destination_parameter == identifier {
                            for undesirable in undesirables {
                                binder.add_string_undesirable(undesirable);
                            }
                            binder.mark_used();
                            break;
                        }
                        if binder.source_parameter.is_some() {
                            undesirables.push(binder.source_parameter.unwrap());
                        }
                        binder.add_identifier_restriction(identifier);
                    }
                } else {

                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                if let Option::Some(identifier) = self.environment.lookup(*variable) {
                    // ...
                } else {

                }
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                if let Option::Some(identifier) = self.environment.lookup(*variable) {
                    // ...
                } else {
                    for binder in self.environment.binders_iter() {
                
                    }
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                if let Option::Some(identifier) = self.environment.lookup(*variable) {
                    // ...
                } else {
                    for binder in self.environment.binders_iter() {
                        let binder = self.binders.get_mut(*binder).unwrap();
                        binder.add_string_restriction(*variable);
                    }
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
            
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
                let (binder_source_parameter_name, binder_destination_parameter_identifier) = {
                    let binder = self.binders.get(binder_expression).unwrap();
                    (binder.source_parameter, binder.destination_parameter)
                };
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
                let (binder_source_parameter_name, binder_destination_parameter_identifier) = /* ... */;
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
                let (binder_source_parameter_name, binder_destination_parameter_identifier) = /* ... */;
                let mut undesirables = Vec::new();
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
                let (binder_source_parameter_name, binder_destination_parameter_identifier) = /* ... */;
                let mut undesirables = Vec::new();
                for sub_binder_expression in
                    self.environment.binders_iter().take(index.into_usize() - 1)
                {
                    let sub_binder = self.binders.get_mut(*sub_binder_expression).unwrap();
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
                let (binder_source_parameter_name, binder_destination_parameter_identifier) = /* ... */;
                let mut undesirables = Vec::new();
                for sub_binder_expression in
                    self.environment.binders_iter().take(index.into_usize() - 1)
                {
                    let sub_binder = self.binders.get_mut(*sub_binder_expression).unwrap();

                    sub_binder.add_identifier_restriction(binder_destination_parameter_identifier);
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
                let (binder_source_parameter_name, binder_destination_parameter_identifier) = /* ... */;
                let mut undesirables = Vec::new();
                for sub_binder_expression in
                    self.environment.binders_iter().take(index.into_usize() - 1)
                {
                    let sub_binder = self.binders.get_mut(*sub_binder_expression).unwrap();

                    sub_binder.add_identifier_restriction(binder_destination_parameter_identifier);

                    if sub_binder.source_parameter.is_some()
                        && binder_source_parameter_name != sub_binder.source_parameter
                    {
                        undesirables.push(sub_binder.source_parameter.unwrap());
                    }
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
                let (binder_source_parameter_name, binder_destination_parameter_identifier) = /* ... */;
                let mut undesirables = /* ... */;
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
                let (binder_source_parameter_name, binder_destination_parameter_identifier) = /* ... */;
                let mut undesirables = /* ... */;
                let binder = self.binders.get_mut(binder_expression).unwrap();
                binder.mark_used();
                for undesirable in undesirables {
                    binder.add_string_undesirable(undesirable);
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Application {
                function,
                arguments,
            } => {
             
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");

  yield* code().code(
    `\
impl<'a> BinderStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Application {
                function,
                arguments,
            } => {
                self.visit(*function);
                for argument in arguments {
                    self.visit(*argument);
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("binder-store-builder");
});
