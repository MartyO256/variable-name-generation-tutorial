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
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {

}`}
    />
  );

  yield* slideTransition(Direction.Right);

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
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

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                let identifier =
                    if let Option::Some(identifier) = self.environment.lookup(*variable) {
                        
                    } else {
                        
                    };
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                let identifier =
                    if let Option::Some(identifier) = self.environment.lookup(*variable) {
                        identifier
                    } else {
                        let identifier = self.identifiers.new_identifier();
                        self.identifiers.set(identifier, *variable);
                        identifier
                    };
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                let identifier = /* ... */;
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                let identifier = /* ... */;
                for binder in self.environment.binders_iter() {
                    
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Variable {
                identifier: variable,
            } => {
                let identifier = /* ... */;
                for binder in self.environment.binders_iter() {
                    let Constraint { parameter, restrictions, used } =
                        self.constraints.get_mut(*binder).unwrap();
                    if *parameter == identifier {
                        *used = true;
                        break;
                    }
                    restrictions.insert(identifier);
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
            
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder = self.environment.lookup_binder(*index);
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder = self.environment.lookup_binder(*index);
                let identifier = {
                    let Constraint {
                        parameter,
                        restrictions: _,
                        used: _,
                    } = self.constraints.get(binder).unwrap();
                    *parameter
                };
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder = self.environment.lookup_binder(*index);
                let identifier = /* ... */;
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder = self.environment.lookup_binder(*index);
                let identifier = /* ... */;
                for sub_binder in self.environment.binders_iter().take(index.into_usize() - 1) {
                    let Constraint {
                        parameter,
                        restrictions,
                        used: _,
                    } = self.constraints.get_mut(*sub_binder).unwrap();
                    restrictions.insert(identifier);
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder = self.environment.lookup_binder(*index);
                let identifier = /* ... */;
                let mut additional_restrictions = Vec::new();
                for sub_binder in self.environment.binders_iter().take(index.into_usize() - 1) {
                    let Constraint {
                        parameter,
                        restrictions,
                        used: _,
                    } = self.constraints.get_mut(*sub_binder).unwrap();
                    restrictions.insert(identifier);
                    additional_restrictions.push(*parameter);
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder = self.environment.lookup_binder(*index);
                let identifier = /* ... */;
                let additional_restrictions = /* ... */;
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessVariable { index } => {
                let binder = self.environment.lookup_binder(*index);
                let identifier = /* ... */;
                let additional_restrictions = /* ... */;
                let Constraint {
                    parameter: _,
                    restrictions,
                    used,
                } = self.constraints.get_mut(binder).unwrap();
                *used = true;
                for additional_restriction in additional_restrictions {
                    restrictions.insert(additional_restriction);
                }
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
        
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
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

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let constraint = Constraint::new(parameter_identifier);
                self.constraints.set(expression, constraint);
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let constraint = Constraint::new(parameter_identifier);
                self.constraints.set(expression, constraint);
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

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let constraint = Constraint::new(parameter_identifier);
                self.constraints.set(expression, constraint);
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

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let constraint = Constraint::new(parameter_identifier);
                self.constraints.set(expression, constraint);
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
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let constraint = Constraint::new(parameter_identifier);
                self.constraints.set(expression, constraint);
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

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::Abstraction { parameter, body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let constraint = Constraint::new(parameter_identifier);
                self.constraints.set(expression, constraint);
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

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessAbstraction { body } => {
            
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
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

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessAbstraction { body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let constraint = Constraint::new(parameter_identifier);
                self.constraints.set(expression, constraint);
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            Expression::NamelessAbstraction { body } => {
                let parameter_identifier = self.identifiers.new_identifier();
                let constraint = Constraint::new(parameter_identifier);
                self.constraints.set(expression, constraint);
                self.environment.shift(expression);
                self.visit(*body);
                self.environment.unshift();
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
    fn visit(&mut self, expression: ExpressionId) {
        match &self.expressions[expression] {
            
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
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

  yield* beginSlide("constraint-store-builder");

  yield* code().code(
    `\
struct ConstraintStoreBuilder<'a> {
    expressions: &'a ExpressionArena,
    identifiers: &'a mut IdentifierArena,
    constraints: ConstraintStore,
    environment: ReferencingEnvironment,
}

impl<'a> ConstraintStoreBuilder<'a> {
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

  yield* beginSlide("constraint-store-builder");
});
