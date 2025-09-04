// Generated from LExpr.g4 by ANTLR 4.13.2
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
use super::lexprlistener::*;
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

		pub const LExpr_MULT:i32=1; 
		pub const LExpr_ADD:i32=2; 
		pub const LExpr_INT:i32=3; 
		pub const LExpr_WS:i32=4;
	pub const LExpr_EOF:i32=EOF;
	pub const RULE_s:usize = 0; 
	pub const RULE_e:usize = 1;
	pub const ruleNames: [&'static str; 2] =  [
		"s", "e"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;3] = [
		None, Some("'*'"), Some("'+'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;5]  = [
		None, Some("MULT"), Some("ADD"), Some("INT"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,LExprParserExt<'input>, I, LExprParserContextType , dyn LExprListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type LExprTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, LExprParserContextType , dyn LExprListener<'input> + 'a>;

/// Parser for LExpr grammar
pub struct LExprParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> LExprParser<'input, I, H>
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
				LExprParserExt{
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

impl<'input, I> LExprParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> LExprParser<'input, I, DefaultErrorStrategy<'input,LExprParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for LExprParser
pub trait LExprParserContext<'input>:
	for<'x> Listenable<dyn LExprListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=LExprParserContextType>
{}

antlr4rust::coerce_from!{ 'input : LExprParserContext<'input> }

impl<'input> LExprParserContext<'input> for TerminalNode<'input,LExprParserContextType> {}
impl<'input> LExprParserContext<'input> for ErrorNode<'input,LExprParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn LExprParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn LExprListener<'input> + 'input }

pub struct LExprParserContextType;
antlr4rust::tid!{LExprParserContextType}

impl<'input> ParserNodeType<'input> for LExprParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn LExprParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for LExprParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for LExprParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct LExprParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> LExprParserExt<'input>{
}
antlr4rust::tid! { LExprParserExt<'a> }

impl<'input> TokenAware<'input> for LExprParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for LExprParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for LExprParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "LExpr.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn LExprParserContext<'input> + 'input)>, rule_index: i32, pred_index: i32,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					1 => LExprParser::<'input,I,_>::e_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> LExprParser<'input, I, DefaultErrorStrategy<'input,LExprParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn e_sempred(_localctx: Option<&EContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 3)
				}
				1=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
}
//------------------- s ----------------
pub type SContextAll<'input> = SContext<'input>;


pub type SContext<'input> = BaseParserRuleContext<'input,SContextExt<'input>>;

#[derive(Clone)]
pub struct SContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LExprParserContext<'input> for SContext<'input>{}

impl<'input,'a> Listenable<dyn LExprListener<'input> + 'a> for SContext<'input>{
		fn enter(&self,listener: &mut (dyn LExprListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_s(self);
		}fn exit(&self,listener: &mut (dyn LExprListener<'input> + 'a)) {
			listener.exit_s(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for SContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_s }
	//fn type_rule_index() -> usize where Self: Sized { RULE_s }
}
antlr4rust::tid!{SContextExt<'a>}

impl<'input> SContextExt<'input>{
	fn new(parent: Option<Rc<dyn LExprParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<SContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait SContextAttrs<'input>: LExprParserContext<'input> + BorrowMut<SContextExt<'input>>{

fn e(&self) -> Option<Rc<EContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SContextAttrs<'input> for SContext<'input>{}

impl<'input, I, H> LExprParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn s(&mut self,)
	-> Result<Rc<SContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_s);
        let mut _localctx: Rc<SContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule e*/
			recog.base.set_state(4);
			recog.e_rec(0)?;

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
//------------------- e ----------------
#[derive(Debug)]
pub enum EContextAll<'input>{
	AddContext(AddContext<'input>),
	MultContext(MultContext<'input>),
	IntContext(IntContext<'input>),
Error(EContext<'input>)
}
antlr4rust::tid!{EContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for EContextAll<'input>{}

impl<'input> LExprParserContext<'input> for EContextAll<'input>{}

impl<'input> Deref for EContextAll<'input>{
	type Target = dyn EContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use EContextAll::*;
		match self{
			AddContext(inner) => inner,
			MultContext(inner) => inner,
			IntContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn LExprListener<'input> + 'a> for EContextAll<'input>{
    fn enter(&self, listener: &mut (dyn LExprListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn LExprListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type EContext<'input> = BaseParserRuleContext<'input,EContextExt<'input>>;

#[derive(Clone)]
pub struct EContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LExprParserContext<'input> for EContext<'input>{}

impl<'input,'a> Listenable<dyn LExprListener<'input> + 'a> for EContext<'input>{
}

impl<'input> CustomRuleContext<'input> for EContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_e }
	//fn type_rule_index() -> usize where Self: Sized { RULE_e }
}
antlr4rust::tid!{EContextExt<'a>}

impl<'input> EContextExt<'input>{
	fn new(parent: Option<Rc<dyn LExprParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EContextAll<'input>> {
		Rc::new(
		EContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait EContextAttrs<'input>: LExprParserContext<'input> + BorrowMut<EContextExt<'input>>{


}

impl<'input> EContextAttrs<'input> for EContext<'input>{}

pub type AddContext<'input> = BaseParserRuleContext<'input,AddContextExt<'input>>;

pub trait AddContextAttrs<'input>: LExprParserContext<'input>{
	fn e_all(&self) ->  Vec<Rc<EContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn e(&self, i: usize) -> Option<Rc<EContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token ADD
	/// Returns `None` if there is no child corresponding to token ADD
	fn ADD(&self) -> Option<Rc<TerminalNode<'input,LExprParserContextType>>> where Self:Sized{
		self.get_token(LExpr_ADD, 0)
	}
}

impl<'input> AddContextAttrs<'input> for AddContext<'input>{}

pub struct AddContextExt<'input>{
	base:EContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{AddContextExt<'a>}

impl<'input> LExprParserContext<'input> for AddContext<'input>{}

impl<'input,'a> Listenable<dyn LExprListener<'input> + 'a> for AddContext<'input>{
	fn enter(&self,listener: &mut (dyn LExprListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Add(self);
	}
}

impl<'input> CustomRuleContext<'input> for AddContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_e }
	//fn type_rule_index() -> usize where Self: Sized { RULE_e }
}

impl<'input> Borrow<EContextExt<'input>> for AddContext<'input>{
	fn borrow(&self) -> &EContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<EContextExt<'input>> for AddContext<'input>{
	fn borrow_mut(&mut self) -> &mut EContextExt<'input> { &mut self.base }
}

impl<'input> EContextAttrs<'input> for AddContext<'input> {}

impl<'input> AddContextExt<'input>{
	fn new(ctx: &dyn EContextAttrs<'input>) -> Rc<EContextAll<'input>>  {
		Rc::new(
			EContextAll::AddContext(
				BaseParserRuleContext::copy_from(ctx,AddContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MultContext<'input> = BaseParserRuleContext<'input,MultContextExt<'input>>;

pub trait MultContextAttrs<'input>: LExprParserContext<'input>{
	fn e_all(&self) ->  Vec<Rc<EContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn e(&self, i: usize) -> Option<Rc<EContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token MULT
	/// Returns `None` if there is no child corresponding to token MULT
	fn MULT(&self) -> Option<Rc<TerminalNode<'input,LExprParserContextType>>> where Self:Sized{
		self.get_token(LExpr_MULT, 0)
	}
}

impl<'input> MultContextAttrs<'input> for MultContext<'input>{}

pub struct MultContextExt<'input>{
	base:EContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MultContextExt<'a>}

impl<'input> LExprParserContext<'input> for MultContext<'input>{}

impl<'input,'a> Listenable<dyn LExprListener<'input> + 'a> for MultContext<'input>{
	fn enter(&self,listener: &mut (dyn LExprListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Mult(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_e }
	//fn type_rule_index() -> usize where Self: Sized { RULE_e }
}

impl<'input> Borrow<EContextExt<'input>> for MultContext<'input>{
	fn borrow(&self) -> &EContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<EContextExt<'input>> for MultContext<'input>{
	fn borrow_mut(&mut self) -> &mut EContextExt<'input> { &mut self.base }
}

impl<'input> EContextAttrs<'input> for MultContext<'input> {}

impl<'input> MultContextExt<'input>{
	fn new(ctx: &dyn EContextAttrs<'input>) -> Rc<EContextAll<'input>>  {
		Rc::new(
			EContextAll::MultContext(
				BaseParserRuleContext::copy_from(ctx,MultContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntContext<'input> = BaseParserRuleContext<'input,IntContextExt<'input>>;

pub trait IntContextAttrs<'input>: LExprParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token INT
	/// Returns `None` if there is no child corresponding to token INT
	fn INT(&self) -> Option<Rc<TerminalNode<'input,LExprParserContextType>>> where Self:Sized{
		self.get_token(LExpr_INT, 0)
	}
}

impl<'input> IntContextAttrs<'input> for IntContext<'input>{}

pub struct IntContextExt<'input>{
	base:EContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IntContextExt<'a>}

impl<'input> LExprParserContext<'input> for IntContext<'input>{}

impl<'input,'a> Listenable<dyn LExprListener<'input> + 'a> for IntContext<'input>{
	fn enter(&self,listener: &mut (dyn LExprListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Int(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_e }
	//fn type_rule_index() -> usize where Self: Sized { RULE_e }
}

impl<'input> Borrow<EContextExt<'input>> for IntContext<'input>{
	fn borrow(&self) -> &EContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<EContextExt<'input>> for IntContext<'input>{
	fn borrow_mut(&mut self) -> &mut EContextExt<'input> { &mut self.base }
}

impl<'input> EContextAttrs<'input> for IntContext<'input> {}

impl<'input> IntContextExt<'input>{
	fn new(ctx: &dyn EContextAttrs<'input>) -> Rc<EContextAll<'input>>  {
		Rc::new(
			EContextAll::IntContext(
				BaseParserRuleContext::copy_from(ctx,IntContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> LExprParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  e(&mut self,)
	-> Result<Rc<EContextAll<'input>>,ANTLRError> {
		self.e_rec(0)
	}

	fn e_rec(&mut self, _p: i32)
	-> Result<Rc<EContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = EContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 2, RULE_e, _p);
	    let mut _localctx: Rc<EContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 2;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			let mut tmp = IntContextExt::new(&**_localctx);
			recog.ctx = Some(tmp.clone());
			_localctx = tmp;
			_prevctx = _localctx.clone();

			recog.base.set_state(7);
			recog.base.match_token(LExpr_INT,&mut recog.err_handler)?;

			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(17);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(1,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(15);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(0,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = MultContextExt::new(&**EContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_e);
							_localctx = tmp;
							recog.base.set_state(9);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(10);
							recog.base.match_token(LExpr_MULT,&mut recog.err_handler)?;

							/*InvokeRule e*/
							recog.base.set_state(11);
							recog.e_rec(4)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = AddContextExt::new(&**EContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_e);
							_localctx = tmp;
							recog.base.set_state(12);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(13);
							recog.base.match_token(LExpr_ADD,&mut recog.err_handler)?;

							/*InvokeRule e*/
							recog.base.set_state(14);
							recog.e_rec(3)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(19);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(1,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

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
const _serializedATN: [i32; 180] = [
	4, 1, 4, 21, 2, 0, 7, 0, 2, 1, 7, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 1, 16, 8, 1, 10, 1, 12, 1, 19, 9, 1, 1, 
	1, 0, 1, 2, 2, 0, 2, 0, 0, 20, 0, 4, 1, 0, 0, 0, 2, 6, 1, 0, 0, 0, 4, 5, 
	3, 2, 1, 0, 5, 1, 1, 0, 0, 0, 6, 7, 6, 1, -1, 0, 7, 8, 5, 3, 0, 0, 8, 17, 
	1, 0, 0, 0, 9, 10, 10, 3, 0, 0, 10, 11, 5, 1, 0, 0, 11, 16, 3, 2, 1, 4, 
	12, 13, 10, 2, 0, 0, 13, 14, 5, 2, 0, 0, 14, 16, 3, 2, 1, 3, 15, 9, 1, 
	0, 0, 0, 15, 12, 1, 0, 0, 0, 16, 19, 1, 0, 0, 0, 17, 15, 1, 0, 0, 0, 17, 
	18, 1, 0, 0, 0, 18, 3, 1, 0, 0, 0, 19, 17, 1, 0, 0, 0, 2, 15, 17
];
