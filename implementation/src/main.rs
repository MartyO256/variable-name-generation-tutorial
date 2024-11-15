extern crate indicatif;
extern crate rand;

use std::{
    fs::File,
    io::{LineWriter, Write},
    path::Path,
    rc::Rc,
    sync::mpsc,
    thread,
    time::Duration,
    usize,
};

use expression::{Expression, ExpressionArena, ExpressionId};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::{thread_rng, Rng};
use referencing_environment::ReferencingEnvironment;
use strings::StringArena;

pub mod strings;

pub mod expression;

pub mod expression_free_variables;
pub mod expression_height;
pub mod expression_locally_nameless;
pub mod expression_named;
pub mod expression_parent;
pub mod expression_size;
pub mod fresh_variable_name_generators;
pub mod parser;
pub mod pretty_print;
pub mod random_expressions;

pub mod alpha_equivalence;
pub mod equality;
pub mod referencing_environment;
pub mod simple_to_named;
pub mod to_locally_nameless;
pub mod to_named;

#[derive(Clone)]
struct SampledExpression {
    expression: ExpressionId,
    size: usize,
    height: usize,
}

impl SampledExpression {
    pub fn new(expression: ExpressionId, size: usize, height: usize) -> SampledExpression {
        SampledExpression {
            expression,
            size,
            height,
        }
    }
}

fn sample_expression<R: Rng>(
    strings: &mut StringArena,
    expressions: &mut ExpressionArena,
    environment: Rc<ReferencingEnvironment>,
    rng: &mut R,
    max_depth: usize,
) -> SampledExpression {
    let expression = Expression::sample(strings, expressions, environment.clone(), rng, max_depth);
    let size = Expression::size(&expressions, expression);
    let height = Expression::height(&expressions, expression);
    SampledExpression::new(expression, size, height)
}

fn is_different(
    expressions: &ExpressionArena,
    environment: Rc<ReferencingEnvironment>,
    sampled_expression: &SampledExpression,
    selected_expressions: &Vec<SampledExpression>,
) -> bool {
    for selected_expression in selected_expressions.iter() {
        if sampled_expression.size == selected_expression.size
            && sampled_expression.height == selected_expression.height
            && Expression::alpha_equivalent(
                (
                    environment.clone(),
                    &expressions,
                    sampled_expression.expression,
                ),
                (
                    environment.clone(),
                    &expressions,
                    selected_expression.expression,
                ),
            )
        {
            return false;
        }
    }
    true
}

fn main() {
    let multi_progress = MultiProgress::new();
    let progress_style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    let depth_range: usize = 12;
    let per_depth_sample_count: usize = 100;

    let (tx, rx) = mpsc::channel();

    let mut threads = Vec::new();
    for max_depth in 0..=depth_range {
        let multi_progress = multi_progress.clone();
        let progress_style = progress_style.clone();
        let tx = tx.clone();
        threads.push(thread::spawn(move || {
            let progress_bar = multi_progress.add(
                ProgressBar::new(per_depth_sample_count as u64)
                    .with_style(progress_style)
                    .with_message(format!(
                        "Generating samples of maximum depth {}...",
                        max_depth
                    )),
            );
            progress_bar.enable_steady_tick(Duration::from_millis(200));

            let mut rng = thread_rng();

            let mut strings = StringArena::new();
            let mut expressions = ExpressionArena::new();
            let environment = Rc::new(ReferencingEnvironment::new());

            let mut selected_expressions: Vec<SampledExpression> = Vec::new();
            for _sample in 0..per_depth_sample_count {
                loop {
                    let sampled_expression = sample_expression(
                        &mut strings,
                        &mut expressions,
                        environment.clone(),
                        &mut rng,
                        max_depth,
                    );
                    if is_different(
                        &expressions,
                        environment.clone(),
                        &sampled_expression,
                        &selected_expressions,
                    ) {
                        selected_expressions.push(sampled_expression.clone());
                        let as_string = Expression::to_string(
                            &strings,
                            &expressions,
                            usize::MAX,
                            sampled_expression.expression,
                        )
                        .unwrap();
                        tx.send(as_string).expect("Failed to send expression");
                        break;
                    }
                }
                progress_bar.inc(1);
            }
        }));
    }

    let output_path = Path::new("expressions.list");
    let output_file = File::create(&output_path).unwrap();
    let mut output_file = LineWriter::new(output_file);

    while let Result::Ok(sampled_expression) = rx.recv() {
        output_file
            .write_all(sampled_expression.as_bytes())
            .unwrap();
        output_file.write_all(b"\n").unwrap();
    }
}
