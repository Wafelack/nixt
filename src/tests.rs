#[cfg(test)]
mod test {
  use crate::{
    core::lexer::*,
    core::parser::*,
    utils::node::*,
    utils::token::{TokenType::*, *},
  };

  fn get_ast(code: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.scan_tokens();
    if lexer.get_errors().is_some() {
      return Err(format!(
        "Lexing errors occured: {:?}",
        lexer.get_errors().unwrap()
      ));
    }
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    if parser.get_errors().is_some() {
      return Err(format!(
        "Parsing errors occured: {:?}",
        parser.get_errors().unwrap()
      ));
    }

    Ok(stringify(&ast, 0))
  }

  #[test]
  fn parse_function() -> Result<(), String> {
    let got = get_ast("(let foo (func (a b c) {}))")?;
    let expected = r#"{
@type : Block
@children : {
  @type : Assignement(Let)
  @children : {
    @type : NodeIdentifier("foo")
    @children : {
      }
    @type : Block
    @children : {
      @type : Func
      @children : {
        @type : Block
        @children : {
          @type : NodeIdentifier("a")
          @children : {
            }
          @type : NodeIdentifier("b")
          @children : {
            }
          @type : NodeIdentifier("c")
          @children : {
            }
          }
        @type : Scope
        @children : {
          }
        }
      }
    }
  }
}"#
      .to_owned();

    assert_eq!(got.trim(), expected.trim());
    Ok(())
  }

  #[test]
  fn parse_operation() -> Result<(), String> {
    let got = get_ast("(+ 5 (- 6 (* 5 (/ 8 2))))")?;
    let expected = r#"{
@type : Block
@children : {
  @type : Operator(Plus)
  @children : {
    @type : NodeNumber(5.0)
    @children : {
      }
    @type : Block
    @children : {
      @type : Operator(Minus)
      @children : {
        @type : NodeNumber(6.0)
        @children : {
          }
        @type : Block
        @children : {
          @type : Operator(Times)
          @children : {
            @type : NodeNumber(5.0)
            @children : {
              }
            @type : Block
            @children : {
              @type : Operator(Div)
              @children : {
                @type : NodeNumber(8.0)
                @children : {
                  }
                @type : NodeNumber(2.0)
                @children : {
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}"#
      .to_owned();

    assert_eq!(got.trim(), expected.trim());
    Ok(())
  }
  #[test]
  fn parse_assignement() -> Result<(), String> {
    let got = get_ast("(let bar 7)(const foo 'bar')")?;
    let expected = r#"{
@type : Block
@children : {
  @type : Assignement(Let)
  @children : {
    @type : NodeIdentifier("bar")
    @children : {
      }
    @type : NodeNumber(7.0)
    @children : {
      }
    }
  }
@type : Block
@children : {
  @type : Assignement(Const)
  @children : {
    @type : NodeIdentifier("foo")
    @children : {
      }
    @type : NodeStr("bar")
    @children : {
      }
    }
  }
}"#
      .to_owned();

    assert_eq!(got.trim(), expected.trim());
    Ok(())
  }
  #[test]
  fn parse_loop() -> Result<(), String> {
    let got = get_ast("{(while (< 5 6) {})}")?;
    let expected = r#"{
@type : Scope
@children : {
  @type : Block
  @children : {
    @type : Loop
    @children : {
      @type : Block
      @children : {
        @type : Operator(Less)
        @children : {
          @type : NodeNumber(5.0)
          @children : {
            }
          @type : NodeNumber(6.0)
          @children : {
            }
          }
        }
      @type : Scope
      @children : {
        }
      }
    }
  }
}"#
      .to_owned();

    assert_eq!(got.trim(), expected.trim());
    Ok(())
  }
}
