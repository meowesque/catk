use crate::{pp::syntax, source};
use std::path::PathBuf;

struct ParseContext<'a> {
  options: &'a super::options::Options,
  source: source::SourceRef,
}

peg::parser! {
  grammar preprocessor(parse_context: &ParseContext<'_>) for source::SourceRef {
    rule __ = " " / "\t"
    rule ___ = ((__ / "\n")+)

    rule path() -> PathBuf
      =
    { todo!() }

    rule name() -> String 
      = 
    { todo!() }

    rule local_include_path() -> syntax::IncludePath
      = begin:position!() "\"" path:path() "\""  end:position!()
    {
      syntax::IncludePath::local(
        source::RegionRef::new(parse_context.source.clone(), begin, end),
        path
      )
    }

    rule global_include_path() -> syntax::IncludePath
      = begin:position!() "<" path:path() ">" end:position!()
    {
      syntax::IncludePath::global(
        source::RegionRef::new(parse_context.source.clone(), begin, end),
        path
      )
    }

    rule include_path() -> syntax::IncludePath
      = local_include_path() / global_include_path()

    rule define_parameters() -> ()
      = 
    { todo!() }

    rule include_directive() -> syntax::Item
      = __ begin:position!() "#" __ "include" __ include_path:include_path() end:position!() ___
    { todo!() }

    rule define_directive() -> syntax::Item
      = __ begin:position!() "#" __ "define" __ name:name() parameters:define_parameters()? __ end:position!() ___ 
    { todo!() }

    rule directive() -> syntax::Item
      = include_directive()
  }
}
