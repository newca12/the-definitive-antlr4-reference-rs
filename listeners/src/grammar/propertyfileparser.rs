// Generated from PropertyFile.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr4rust::PredictionContextCache;
use antlr4rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr4rust::token_stream::TokenStream;
use antlr4rust::TokenSource;
use antlr4rust::parser_atn_simulator::ParserATNSimulator;
use antlr4rust::errors::*;
use antlr4rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::atn::{ATN, INVALID_ALT};
use antlr4rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr4rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr4rust::tree::*;
use antlr4rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr4rust::int_stream::EOF;
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr4rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::propertyfilelistener::*;
use super::propertyfilevisitor::*;

use antlr4rust::lazy_static;
use antlr4rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const PropertyFile_T__0:i32=1; 
		pub const PropertyFile_T__1:i32=2; 
		pub const PropertyFile_ID:i32=3; 
		pub const PropertyFile_STRING:i32=4;
	pub const PropertyFile_EOF:i32=EOF;
	pub const RULE_file:usize = 0; 
	pub const RULE_prop:usize = 1;
	pub const ruleNames: [&'static str; 2] =  [
		"file", "prop"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;3] = [
		None, Some("'='"), Some("'\\n'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;5]  = [
		None, None, None, Some("ID"), Some("STRING")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,PropertyFileParserExt<'input>, I, PropertyFileParserContextType , dyn PropertyFileListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type PropertyFileTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, PropertyFileParserContextType , dyn PropertyFileListener<'input> + 'a>;

/// Parser for PropertyFile grammar
pub struct PropertyFileParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> PropertyFileParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr4rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				PropertyFileParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> PropertyFileParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> PropertyFileParser<'input, I, DefaultErrorStrategy<'input,PropertyFileParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for PropertyFileParser
pub trait PropertyFileParserContext<'input>:
	for<'x> Listenable<dyn PropertyFileListener<'input> + 'x > + 
	for<'x> Visitable<dyn PropertyFileVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=PropertyFileParserContextType>
{}

antlr4rust::coerce_from!{ 'input : PropertyFileParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn PropertyFileParserContext<'input> + 'input
where
    T: PropertyFileVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn PropertyFileVisitor<'input> + 'x))
    }
}

impl<'input> PropertyFileParserContext<'input> for TerminalNode<'input,PropertyFileParserContextType> {}
impl<'input> PropertyFileParserContext<'input> for ErrorNode<'input,PropertyFileParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn PropertyFileParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn PropertyFileListener<'input> + 'input }

pub struct PropertyFileParserContextType;
antlr4rust::tid!{PropertyFileParserContextType}

impl<'input> ParserNodeType<'input> for PropertyFileParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn PropertyFileParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for PropertyFileParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for PropertyFileParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct PropertyFileParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> PropertyFileParserExt<'input>{
}
antlr4rust::tid! { PropertyFileParserExt<'a> }

impl<'input> TokenAware<'input> for PropertyFileParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for PropertyFileParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for PropertyFileParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "PropertyFile.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- file ----------------
pub type FileContextAll<'input> = FileContext<'input>;


pub type FileContext<'input> = BaseParserRuleContext<'input,FileContextExt<'input>>;

#[derive(Clone)]
pub struct FileContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PropertyFileParserContext<'input> for FileContext<'input>{}

impl<'input,'a> Listenable<dyn PropertyFileListener<'input> + 'a> for FileContext<'input>{
		fn enter(&self,listener: &mut (dyn PropertyFileListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_file(self);
		}
		fn exit(&self,listener: &mut (dyn PropertyFileListener<'input> + 'a)) {
			listener.exit_file(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn PropertyFileVisitor<'input> + 'a> for FileContext<'input>{
	fn accept(&self,visitor: &mut (dyn PropertyFileVisitor<'input> + 'a)) {
		visitor.visit_file(self);
	}
}

impl<'input> CustomRuleContext<'input> for FileContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PropertyFileParserContextType;
	fn get_rule_index(&self) -> usize { RULE_file }
	//fn type_rule_index() -> usize where Self: Sized { RULE_file }
}
antlr4rust::tid!{FileContextExt<'a>}

impl<'input> FileContextExt<'input>{
	fn new(parent: Option<Rc<dyn PropertyFileParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FileContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FileContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait FileContextAttrs<'input>: PropertyFileParserContext<'input> + BorrowMut<FileContextExt<'input>>{

fn prop_all(&self) ->  Vec<Rc<PropContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn prop(&self, i: usize) -> Option<Rc<PropContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> FileContextAttrs<'input> for FileContext<'input>{}

impl<'input, I, H> PropertyFileParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn file(&mut self,)
	-> Result<Rc<FileContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FileContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_file);
        let mut _localctx: Rc<FileContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(5); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule prop*/
				recog.base.set_state(4);
				recog.prop()?;

				}
				}
				recog.base.set_state(7); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==PropertyFile_ID) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- prop ----------------
pub type PropContextAll<'input> = PropContext<'input>;


pub type PropContext<'input> = BaseParserRuleContext<'input,PropContextExt<'input>>;

#[derive(Clone)]
pub struct PropContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> PropertyFileParserContext<'input> for PropContext<'input>{}

impl<'input,'a> Listenable<dyn PropertyFileListener<'input> + 'a> for PropContext<'input>{
		fn enter(&self,listener: &mut (dyn PropertyFileListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_prop(self);
		}
		fn exit(&self,listener: &mut (dyn PropertyFileListener<'input> + 'a)) {
			listener.exit_prop(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn PropertyFileVisitor<'input> + 'a> for PropContext<'input>{
	fn accept(&self,visitor: &mut (dyn PropertyFileVisitor<'input> + 'a)) {
		visitor.visit_prop(self);
	}
}

impl<'input> CustomRuleContext<'input> for PropContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = PropertyFileParserContextType;
	fn get_rule_index(&self) -> usize { RULE_prop }
	//fn type_rule_index() -> usize where Self: Sized { RULE_prop }
}
antlr4rust::tid!{PropContextExt<'a>}

impl<'input> PropContextExt<'input>{
	fn new(parent: Option<Rc<dyn PropertyFileParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PropContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PropContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait PropContextAttrs<'input>: PropertyFileParserContext<'input> + BorrowMut<PropContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,PropertyFileParserContextType>>> where Self:Sized{
	self.get_token(PropertyFile_ID, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,PropertyFileParserContextType>>> where Self:Sized{
	self.get_token(PropertyFile_STRING, 0)
}

}

impl<'input> PropContextAttrs<'input> for PropContext<'input>{}

impl<'input, I, H> PropertyFileParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn prop(&mut self,)
	-> Result<Rc<PropContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PropContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_prop);
        let mut _localctx: Rc<PropContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(9);
			recog.base.match_token(PropertyFile_ID,&mut recog.err_handler)?;

			recog.base.set_state(10);
			recog.base.match_token(PropertyFile_T__0,&mut recog.err_handler)?;

			recog.base.set_state(11);
			recog.base.match_token(PropertyFile_STRING,&mut recog.err_handler)?;

			recog.base.set_state(12);
			recog.base.match_token(PropertyFile_T__1,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
	lazy_static!{
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.into_iter()));
    static ref _decision_to_DFA: Arc<Vec<antlr4rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len() as i32;
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i,
            ).into())
        }
        Arc::new(dfa)
    };
    }
const _serializedATN: [i32; 124] = [
	4, 1, 4, 15, 2, 0, 7, 0, 2, 1, 7, 1, 1, 0, 4, 0, 6, 8, 0, 11, 0, 12, 0, 
	7, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 2, 0, 2, 0, 0, 13, 0, 5, 1, 
	0, 0, 0, 2, 9, 1, 0, 0, 0, 4, 6, 3, 2, 1, 0, 5, 4, 1, 0, 0, 0, 6, 7, 1, 
	0, 0, 0, 7, 5, 1, 0, 0, 0, 7, 8, 1, 0, 0, 0, 8, 1, 1, 0, 0, 0, 9, 10, 5, 
	3, 0, 0, 10, 11, 5, 1, 0, 0, 11, 12, 5, 4, 0, 0, 12, 13, 5, 2, 0, 0, 13, 
	3, 1, 0, 0, 0, 1, 7
];
