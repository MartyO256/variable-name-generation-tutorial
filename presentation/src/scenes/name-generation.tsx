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
    strings: &'a mut StringArena,
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a> {
    strings: &'a mut StringArena,
    source: &'a ExpressionArena,
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
    binders: BinderStore,
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
    binders: BinderStore,
    environment: ReferencingEnvironment,
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> {
    strings: &'a mut StringArena,
    source: &'a ExpressionArena,
    destination: &'a mut ExpressionArena,
    identifiers: IdentifierArena,
    binders: BinderStore,
    environment: ReferencingEnvironment,
    variable_name_generator: G,
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* all(
    code().fontSize(30, 1),
    code().code(
      `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {

}`,
      1
    )
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn evaluate_constraint_set(&self, constraints: &HashSet<Constraint>)  -> HashSet<StringId> {
        
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn evaluate_constraint_set(&self, constraints: &HashSet<Constraint>)  -> HashSet<StringId> {
        let mut identifiers = HashSet::new();
        
        identifiers
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn evaluate_constraint_set(&self, constraints: &HashSet<Constraint>)  -> HashSet<StringId> {
        let mut identifiers = HashSet::new();
        for constraint in constraints {
            
        }
        identifiers
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn evaluate_constraint_set(&self, constraints: &HashSet<Constraint>)  -> HashSet<StringId> {
        let mut identifiers = HashSet::new();
        for constraint in constraints {
            if let Option::Some(string) = constraint.evaluate(&self.identifiers) {
                identifiers.insert(string);
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {

            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter =
                    if let Option::Some(name) = source_parameter {
                        
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter =
                    if let Option::Some(name) = source_parameter {
                        let restrictions = self.evaluate_constraint_set(&binder.restrictions);
                        if restrictions.contains(name) {
                            
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter =
                    if let Option::Some(name) = source_parameter {
                        let restrictions = self.evaluate_constraint_set(&binder.restrictions);
                        if restrictions.contains(name) {
                            let undesirables = self.evaluate_constraint_set(&binder.undesirables);
                            let new_name = self
                                .variable_name_generator
                                .generate_admissible_name(self.strings, |name| {
                                    !restrictions.contains(&name) && !undesirables.contains(&name)
                                });
                            self.identifiers.set(binder.destination_parameter, new_name);
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter =
                    if let Option::Some(name) = source_parameter {
                        let restrictions = self.evaluate_constraint_set(&binder.restrictions);
                        if restrictions.contains(name) {
                            // ...
                        } else {
                            
                        }
                    };
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter =
                    if let Option::Some(name) = source_parameter {
                        let restrictions = self.evaluate_constraint_set(&binder.restrictions);
                        if restrictions.contains(name) {
                            // ...
                        } else {
                            self.identifiers.set(binder.destination_parameter, *name);
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter =
                    if let Option::Some(name) = source_parameter {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter =
                    if let Option::Some(name) = source_parameter {
                        // ...
                    } else if binder.is_used() {
                    
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter =
                    if let Option::Some(name) = source_parameter {
                        // ...
                    } else if binder.is_used() {
                        let restrictions = self.evaluate_constraint_set(&binder.restrictions);
                        let undesirables = self.evaluate_constraint_set(&binder.undesirables);
                        let name = self
                            .variable_name_generator
                            .generate_admissible_name(self.strings, |name| {
                                !restrictions.contains(&name) && !undesirables.contains(&name)
                            });
                        self.identifiers.set(binder.destination_parameter, name);
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter =
                    if let Option::Some(name) = source_parameter {
                        // ...
                    } else if binder.is_used() {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter =
                    if let Option::Some(name) = source_parameter {
                        // ...
                    } else if binder.is_used() {
                        // ...
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter = /* ... */;
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter = /* ... */;
                match source_parameter {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter = /* ... */;
                match source_parameter {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::Abstraction { parameter: source_parameter, body } => {
                let binder = self.binders.get(expression).unwrap();
                let chosen_parameter = /* ... */;
                match source_parameter {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessAbstraction { body } => {
                let binder = self.binders.get(expression).unwrap();
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessAbstraction { body } => {
                let binder = self.binders.get(expression).unwrap();
                let parameter = if binder.is_used() {
                    let restrictions = self.evaluate_constraint_set(&binder.restrictions);
                    let undesirables = self.evaluate_constraint_set(&binder.undesirables);
                    let name = self
                        .variable_name_generator
                        .generate_admissible_name(self.strings, |name| {
                            !restrictions.contains(&name) && !undesirables.contains(&name)
                        });
                    self.identifiers.set(binder.destination_parameter, name);
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessAbstraction { body } => {
                let binder = self.binders.get(expression).unwrap();
                let parameter = /* ... */;
            }
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessAbstraction { body } => {
                let binder = self.binders.get(expression).unwrap();
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {

        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
                let binder = self.binders.get(binder_expression).unwrap();
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
                let binder = self.binders.get(binder_expression).unwrap();
                let name = self
                    .identifiers
                    .lookup(binder.destination_parameter)
                    .unwrap();
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            Expression::NamelessVariable { index } => {
                let binder_expression = self.environment.lookup_binder(*index);
                let binder = self.binders.get(binder_expression).unwrap();
                let name = self
                    .identifiers
                    .lookup(binder.destination_parameter)
                    .unwrap();
                self.destination.variable(name)
            }
        }
    }
}`,
    1
  );

  yield* beginSlide("name-generation");

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
    fn convert_to_named(&mut self, expression: ExpressionId) -> ExpressionId {
        match &self.source[expression] {
            
        }
    }
}`,
    1
  );

  yield* code().code(
    `\
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
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
struct NameGeneration<'a, G: AdmissibleVariableNameGenerator> { /* ... */ }

impl<'a, G: AdmissibleVariableNameGenerator> NameGeneration<'a, G> {
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
