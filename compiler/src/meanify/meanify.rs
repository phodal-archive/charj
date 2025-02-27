use crate::{ControlFlowGraph, Namespace};
use cjc_hir::{Builtin, Expression, Function, Statement};
use cjc_mir::instruction::ExprKind;

pub fn meanify(ns: &mut Namespace) {
    #[allow(unused_assignments)]
    let mut cfg_no = 0;
    let mut all_cfg = Vec::new();

    cfg_no = ns.functions.len();
    all_cfg.resize(cfg_no, ControlFlowGraph::placeholder());

    let function_no = 0;
    for _cfg in all_cfg {
        function_cfg(function_no, ns);
    }
}

pub fn function_cfg(function_no: usize, ns: &mut Namespace) {
    let func = &ns.functions[function_no];

    let func_name = &func.name;
    let mut cfg = ControlFlowGraph::new(func_name.to_string());
    cfg.params = func.params.clone();
    cfg.returns = func.returns.clone();

    for stmt in &func.body {
        statement_cfg(stmt, func, &mut cfg, ns)
    }

    ns.cfgs.push(cfg);
}

pub fn statement_cfg(
    stmt: &Statement,
    _func: &Function,
    cfg: &mut ControlFlowGraph,
    ns: &Namespace,
) {
    match stmt {
        Statement::VariableDecl { location: _ } => {
            // todo
        }
        Statement::Expression {
            location: _,
            expression: expr,
        } => {
            expression_cfg(expr, cfg, ns);
        }
    }
}

pub fn expression_cfg(expr: &Expression, cfg: &mut ControlFlowGraph, ns: &Namespace) -> Expression {
    match expr {
        Expression::Placeholder => Expression::Placeholder,
        Expression::StringLiteral {
            location: _,
            value: _,
        } => {
            // cfg.emit(ExprKind::Var {
            //     value: value.to_string(),
            // });
            expr.clone()
        }
        Expression::BytesLiteral { .. } => Expression::Placeholder,
        Expression::InternalFunctionCall { .. } => Expression::Placeholder,
        Expression::Builtin {
            location: _,
            types: _,
            builtin,
            args,
        } => match builtin {
            Builtin::Assert => Expression::Placeholder,
            Builtin::Print => {
                let expr = expression_cfg(&args[0], cfg, ns);
                let mut val = "".to_string();
                match expr {
                    Expression::StringLiteral { location: _, value } => {
                        val = value;
                    }
                    _ => {}
                }
                cfg.emit(ExprKind::Print { value: val });
                Expression::Placeholder
            }
        },
        _ => expr.clone(),
    }
}
