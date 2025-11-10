// Generated from ArrayInit.g4 by ANTLR 4.13.2
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
use super::arrayinitlistener::*;
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

		pub const ArrayInit_T__0:i32=1; 
		pub const ArrayInit_T__1:i32=2; 
		pub const ArrayInit_T__2:i32=3; 
		pub const ArrayInit_INT:i32=4; 
		pub const ArrayInit_WS:i32=5;
	pub const ArrayInit_EOF:i32=EOF;
	pub const RULE_init:usize = 0; 
	pub const RULE_value:usize = 1;
	pub const ruleNames: [&'static str; 2] =  [
		"init", "value"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;4] = [
		None, Some("'{'"), Some("','"), Some("'}'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;6]  = [
		None, None, None, None, Some("INT"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,ArrayInitParserExt<'input>, I, ArrayInitParserContextType , dyn ArrayInitListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type ArrayInitTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, ArrayInitParserContextType , dyn ArrayInitListener<'input> + 'a>;

/// Parser for ArrayInit grammar
pub struct ArrayInitParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >,
}

impl<'input, I> ArrayInitParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) -> Self {
		antlr4rust::recognizer::check_version("0","5");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				ArrayInitParserExt{
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

impl<'input, I> ArrayInitParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> ArrayInitParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for ArrayInitParser
pub trait ArrayInitParserContext<'input>:
	for<'x> Listenable<dyn ArrayInitListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=ArrayInitParserContextType>
{}

antlr4rust::coerce_from!{ 'input : ArrayInitParserContext<'input> }

impl<'input> ArrayInitParserContext<'input> for TerminalNode<'input,ArrayInitParserContextType> {}
impl<'input> ArrayInitParserContext<'input> for ErrorNode<'input,ArrayInitParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn ArrayInitParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn ArrayInitListener<'input> + 'input }

pub struct ArrayInitParserContextType;
antlr4rust::tid!{ArrayInitParserContextType}

impl<'input> ParserNodeType<'input> for ArrayInitParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn ArrayInitParserContext<'input> + 'input;
}

impl<'input, I> Deref for ArrayInitParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for ArrayInitParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct ArrayInitParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> ArrayInitParserExt<'input>{
}
antlr4rust::tid! { ArrayInitParserExt<'a> }

impl<'input> TokenAware<'input> for ArrayInitParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for ArrayInitParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for ArrayInitParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "ArrayInit.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- init ----------------
pub type InitContextAll<'input> = InitContext<'input>;


pub type InitContext<'input> = BaseParserRuleContext<'input,InitContextExt<'input>>;

#[derive(Clone)]
pub struct InitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ArrayInitParserContext<'input> for InitContext<'input>{}

impl<'input,'a> Listenable<dyn ArrayInitListener<'input> + 'a> for InitContext<'input>{
		fn enter(&self,listener: &mut (dyn ArrayInitListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_init(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn ArrayInitListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_init(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for InitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ArrayInitParserContextType;
	fn get_rule_index(&self) -> usize { RULE_init }
	//fn type_rule_index() -> usize where Self: Sized { RULE_init }
}
antlr4rust::tid!{InitContextExt<'a>}

impl<'input> InitContextExt<'input>{
	fn new(parent: Option<Rc<dyn ArrayInitParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<InitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait InitContextAttrs<'input>: ArrayInitParserContext<'input> + BorrowMut<InitContextExt<'input>>{

fn value_all(&self) ->  Vec<Rc<ValueContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn value(&self, i: usize) -> Option<Rc<ValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> InitContextAttrs<'input> for InitContext<'input>{}

impl<'input, I> ArrayInitParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn init(&mut self,)
	-> Result<Rc<InitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_init);
        let mut _localctx: Rc<InitContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(4);
			recog.base.match_token(ArrayInit_T__0,&mut recog.err_handler)?;

			/*InvokeRule value*/
			recog.base.set_state(5);
			recog.value()?;

			recog.base.set_state(10);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ArrayInit_T__1 {
				{
				{
				recog.base.set_state(6);
				recog.base.match_token(ArrayInit_T__1,&mut recog.err_handler)?;

				/*InvokeRule value*/
				recog.base.set_state(7);
				recog.value()?;

				}
				}
				recog.base.set_state(12);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(13);
			recog.base.match_token(ArrayInit_T__2,&mut recog.err_handler)?;

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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- value ----------------
pub type ValueContextAll<'input> = ValueContext<'input>;


pub type ValueContext<'input> = BaseParserRuleContext<'input,ValueContextExt<'input>>;

#[derive(Clone)]
pub struct ValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ArrayInitParserContext<'input> for ValueContext<'input>{}

impl<'input,'a> Listenable<dyn ArrayInitListener<'input> + 'a> for ValueContext<'input>{
		fn enter(&self,listener: &mut (dyn ArrayInitListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_value(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn ArrayInitListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_value(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ArrayInitParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}
antlr4rust::tid!{ValueContextExt<'a>}

impl<'input> ValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn ArrayInitParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ValueContextAttrs<'input>: ArrayInitParserContext<'input> + BorrowMut<ValueContextExt<'input>>{

fn init(&self) -> Option<Rc<InitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,ArrayInitParserContextType>>> where Self:Sized{
	self.get_token(ArrayInit_INT, 0)
}

}

impl<'input> ValueContextAttrs<'input> for ValueContext<'input>{}

impl<'input, I> ArrayInitParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn value(&mut self,)
	-> Result<Rc<ValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_value);
        let mut _localctx: Rc<ValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(17);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			ArrayInit_T__0 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule init*/
					recog.base.set_state(15);
					recog.init()?;

					}
				}

			ArrayInit_INT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(16);
					recog.base.match_token(ArrayInit_INT,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
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
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
	lazy_static!{
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
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
	static ref _serializedATN: Vec<i32> = vec![
		4, 1, 5, 20, 2, 0, 7, 0, 2, 1, 7, 1, 1, 0, 1, 0, 1, 0, 1, 0, 5, 0, 9, 
		8, 0, 10, 0, 12, 0, 12, 9, 0, 1, 0, 1, 0, 1, 1, 1, 1, 3, 1, 18, 8, 1, 
		1, 1, 0, 0, 2, 0, 2, 0, 0, 19, 0, 4, 1, 0, 0, 0, 2, 17, 1, 0, 0, 0, 4, 
		5, 5, 1, 0, 0, 5, 10, 3, 2, 1, 0, 6, 7, 5, 2, 0, 0, 7, 9, 3, 2, 1, 0, 
		8, 6, 1, 0, 0, 0, 9, 12, 1, 0, 0, 0, 10, 8, 1, 0, 0, 0, 10, 11, 1, 0, 
		0, 0, 11, 13, 1, 0, 0, 0, 12, 10, 1, 0, 0, 0, 13, 14, 5, 3, 0, 0, 14, 
		1, 1, 0, 0, 0, 15, 18, 3, 0, 0, 0, 16, 18, 5, 4, 0, 0, 17, 15, 1, 0, 0, 
		0, 17, 16, 1, 0, 0, 0, 18, 3, 1, 0, 0, 0, 2, 10, 17
	];
}
