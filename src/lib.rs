mod ir_op;
#[test]
fn test_nop() {
    use crate::ir_op::*;
    let args: [Type; 0] = [];
    let sig: (&[Type], Type) = (&args, Type::Void);
    let ops = [OpKind::Ret];
    let method = Method::from_ops(sig, &ops).expect("Could not compile method `Nop`");
}
#[test]
fn test_add_i32() {
    use crate::ir_op::*;
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::I32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    let method = Method::from_ops(sig, &ops).expect("Could not compile  method  `Add`");
}
#[test]
fn test_wrong_return() {
    use crate::ir_op::*;
    let args: [Type; 2] = [Type::I32, Type::I32];
    let sig: (&[Type], Type) = (&args, Type::F32);
    let ops = [OpKind::LDArg(0), OpKind::LDArg(1), OpKind::Add, OpKind::Ret];
    if let Err(kind) = Method::from_ops(sig, &ops) {
        if let MethodIRError::WrongReturnType { expected, got } = kind {
            assert_eq!(
                expected,
                Type::F32,
                "Error should notify that return type should have been F32"
            );
            assert_eq!(
                got,
                Type::I32,
                "Error should notify that return type was I32"
            );
        } else {
            panic!("Expected an return type error!");
        }
    } else {
        panic!("Expected an return type error!");
    }
}
#[test]
fn test_mag_2_f32() {
    use crate::ir_op::*;
    let args: [Type; 3] = [Type::F32, Type::F32, Type::F32];
    let sig: (&[Type], Type) = (&args, Type::F32);
    let ops = [
        OpKind::LDArg(0),
        OpKind::LDArg(0),
        OpKind::Mul,
        OpKind::LDArg(1),
        OpKind::LDArg(1),
        OpKind::Mul,
        OpKind::Add,
        OpKind::LDArg(2),
        OpKind::LDArg(2),
        OpKind::Mul,
        OpKind::Add,
        OpKind::Ret,
    ];
    let method = Method::from_ops(sig, &ops).expect("Could not compile method `Mag2`");
}
