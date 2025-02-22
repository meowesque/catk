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
      = path_parts:((['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | ' ']+) ++ "/")
    { path_parts.into_iter().flatten().collect::<String>().into() }

    rule name() -> String
      = name_parts:(['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | ' ']+)
    { name_parts.into_iter().collect() }

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

    rule define_directive_parameter_va_args() -> syntax::DefineDirectiveParameter
      = "..."
    { syntax::DefineDirectiveParameter::VaArgs }

    rule define_directive_parameter_name() -> syntax::DefineDirectiveParameter
      = name:name()
    { syntax::DefineDirectiveParameter::Name(name) }

    rule define_directive_parameter() -> syntax::DefineDirectiveParameter
      = define_directive_parameter_va_args() / define_directive_parameter_name()

    rule define_directive_parameters() -> Vec<syntax::DefineDirectiveParameter>
      = "(" __ parameters:(define_directive_parameter() ++ (__ "," __)) __ ")"
    { parameters }

    rule include_directive() -> syntax::Item
      = __ begin:position!()
        "#" __ "include" __ include_path:include_path()
        end:position!() ___
    { todo!() }

    rule define_directive() -> syntax::Item
      = __ begin:position!()
        "#" __ "define" __ name:name() parameters:define_directive_parameters()? __
        end:position!() ___
    { todo!() }

    rule directive() -> syntax::Item
      = include_directive() / define_directive()
  }
}
