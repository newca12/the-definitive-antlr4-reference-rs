// Generated from Expr.g4 by ANTLR 4.13.2
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
use super::exprlistener::*;
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

		pub const Expr_T__0:i32=1; 
		pub const Expr_T__1:i32=2; 
		pub const Expr_T__2:i32=3; 
		pub const Expr_T__3:i32=4; 
		pub const Expr_T__4:i32=5; 
		pub const Expr_T__5:i32=6; 
		pub const Expr_T__6:i32=7; 
		pub const Expr_ID:i32=8; 
		pub const Expr_INT:i32=9; 
		pub const Expr_NEWLINE:i32=10; 
		pub const Expr_WS:i32=11;
	pub const Expr_EOF:i32=EOF;
	pub const RULE_prog:usize = 0; 
	pub const RULE_stat:usize = 1; 
	pub const RULE_expr:usize = 2;
	pub const ruleNames: [&'static str; 3] =  [
		"prog", "stat", "expr"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;8] = [
		None, Some("'='"), Some("'*'"), Some("'/'"), Some("'+'"), Some("'-'"), 
		Some("'('"), Some("')'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;12]  = [
		None, None, None, None, None, None, None, None, Some("ID"), Some("INT"), 
		Some("NEWLINE"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,ExprParserExt<'input>, I, ExprParserContextType , dyn ExprListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type ExprTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, ExprParserContextType , dyn ExprListener<'input> + 'a>;

/// Parser for Expr grammar
pub struct ExprParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> ExprParser<'input, I, H>
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
				ExprParserExt{
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

impl<'input, I> ExprParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> ExprParser<'input, I, DefaultErrorStrategy<'input,ExprParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for ExprParser
pub trait ExprParserContext<'input>:
	for<'x> Listenable<dyn ExprListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=ExprParserContextType>
{}

antlr4rust::coerce_from!{ 'input : ExprParserContext<'input> }

impl<'input> ExprParserContext<'input> for TerminalNode<'input,ExprParserContextType> {}
impl<'input> ExprParserContext<'input> for ErrorNode<'input,ExprParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn ExprParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn ExprListener<'input> + 'input }

pub struct ExprParserContextType;
antlr4rust::tid!{ExprParserContextType}

impl<'input> ParserNodeType<'input> for ExprParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn ExprParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for ExprParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for ExprParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct ExprParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> ExprParserExt<'input>{
}
antlr4rust::tid! { ExprParserExt<'a> }

impl<'input> TokenAware<'input> for ExprParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for ExprParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for ExprParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "Expr.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn ExprParserContext<'input> + 'input)>, rule_index: i32, pred_index: i32,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					2 => ExprParser::<'input,I,_>::expr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> ExprParser<'input, I, DefaultErrorStrategy<'input,ExprParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expr_sempred(_localctx: Option<&ExprContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 5)
				}
				1=>{
					recog.precpred(None, 4)
				}
			_ => true
		}
	}
}
//------------------- prog ----------------
pub type ProgContextAll<'input> = ProgContext<'input>;


pub type ProgContext<'input> = BaseParserRuleContext<'input,ProgContextExt<'input>>;

#[derive(Clone)]
pub struct ProgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ExprParserContext<'input> for ProgContext<'input>{}

impl<'input,'a> Listenable<dyn ExprListener<'input> + 'a> for ProgContext<'input>{
		fn enter(&self,listener: &mut (dyn ExprListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_prog(self);
		}fn exit(&self,listener: &mut (dyn ExprListener<'input> + 'a)) {
			listener.exit_prog(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ProgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_prog }
	//fn type_rule_index() -> usize where Self: Sized { RULE_prog }
}
antlr4rust::tid!{ProgContextExt<'a>}

impl<'input> ProgContextExt<'input>{
	fn new(parent: Option<Rc<dyn ExprParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ProgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ProgContextAttrs<'input>: ExprParserContext<'input> + BorrowMut<ProgContextExt<'input>>{

fn stat_all(&self) ->  Vec<Rc<StatContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stat(&self, i: usize) -> Option<Rc<StatContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProgContextAttrs<'input> for ProgContext<'input>{}

impl<'input, I, H> ExprParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn prog(&mut self,)
	-> Result<Rc<ProgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_prog);
        let mut _localctx: Rc<ProgContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(7); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule stat*/
				recog.base.set_state(6);
				recog.stat()?;

				}
				}
				recog.base.set_state(9); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 1856) != 0)) {break}
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
//------------------- stat ----------------
pub type StatContextAll<'input> = StatContext<'input>;


pub type StatContext<'input> = BaseParserRuleContext<'input,StatContextExt<'input>>;

#[derive(Clone)]
pub struct StatContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ExprParserContext<'input> for StatContext<'input>{}

impl<'input,'a> Listenable<dyn ExprListener<'input> + 'a> for StatContext<'input>{
		fn enter(&self,listener: &mut (dyn ExprListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stat(self);
		}fn exit(&self,listener: &mut (dyn ExprListener<'input> + 'a)) {
			listener.exit_stat(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for StatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stat }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stat }
}
antlr4rust::tid!{StatContextExt<'a>}

impl<'input> StatContextExt<'input>{
	fn new(parent: Option<Rc<dyn ExprParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StatContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StatContextAttrs<'input>: ExprParserContext<'input> + BorrowMut<StatContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NEWLINE
/// Returns `None` if there is no child corresponding to token NEWLINE
fn NEWLINE(&self) -> Option<Rc<TerminalNode<'input,ExprParserContextType>>> where Self:Sized{
	self.get_token(Expr_NEWLINE, 0)
}
/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,ExprParserContextType>>> where Self:Sized{
	self.get_token(Expr_ID, 0)
}

}

impl<'input> StatContextAttrs<'input> for StatContext<'input>{}

impl<'input, I, H> ExprParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stat(&mut self,)
	-> Result<Rc<StatContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_stat);
        let mut _localctx: Rc<StatContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(20);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule expr*/
					recog.base.set_state(11);
					recog.expr_rec(0)?;

					recog.base.set_state(12);
					recog.base.match_token(Expr_NEWLINE,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(14);
					recog.base.match_token(Expr_ID,&mut recog.err_handler)?;

					recog.base.set_state(15);
					recog.base.match_token(Expr_T__0,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(16);
					recog.expr_rec(0)?;

					recog.base.set_state(17);
					recog.base.match_token(Expr_NEWLINE,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(19);
					recog.base.match_token(Expr_NEWLINE,&mut recog.err_handler)?;

					}
				}

				_ => {}
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
//------------------- expr ----------------
pub type ExprContextAll<'input> = ExprContext<'input>;


pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ExprParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn ExprListener<'input> + 'a> for ExprContext<'input>{
		fn enter(&self,listener: &mut (dyn ExprListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr(self);
		}fn exit(&self,listener: &mut (dyn ExprListener<'input> + 'a)) {
			listener.exit_expr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr4rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn ExprParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ExprContextAttrs<'input>: ExprParserContext<'input> + BorrowMut<ExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,ExprParserContextType>>> where Self:Sized{
	self.get_token(Expr_INT, 0)
}
/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,ExprParserContextType>>> where Self:Sized{
	self.get_token(Expr_ID, 0)
}
fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

impl<'input, I, H> ExprParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		self.expr_rec(0)
	}

	fn expr_rec(&mut self, _p: i32)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 4, RULE_expr, _p);
	    let mut _localctx: Rc<ExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 4;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(29);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			Expr_INT 
				=> {
					{
					recog.base.set_state(23);
					recog.base.match_token(Expr_INT,&mut recog.err_handler)?;

					}
				}

			Expr_ID 
				=> {
					{
					recog.base.set_state(24);
					recog.base.match_token(Expr_ID,&mut recog.err_handler)?;

					}
				}

			Expr_T__5 
				=> {
					{
					recog.base.set_state(25);
					recog.base.match_token(Expr_T__5,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(26);
					recog.expr_rec(0)?;

					recog.base.set_state(27);
					recog.base.match_token(Expr_T__6,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(39);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(37);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExprContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(31);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(32);
							_la = recog.base.input.la(1);
							if { !(_la==Expr_T__1 || _la==Expr_T__2) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(33);
							recog.expr_rec(6)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExprContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(34);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(35);
							_la = recog.base.input.la(1);
							if { !(_la==Expr_T__3 || _la==Expr_T__4) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(36);
							recog.expr_rec(5)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(41);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
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
const _serializedATN: [i32; 396] = [
	4, 1, 11, 43, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 1, 0, 4, 0, 8, 8, 0, 
	11, 0, 12, 0, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
	3, 1, 21, 8, 1, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 3, 2, 30, 8, 
	2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 5, 2, 38, 8, 2, 10, 2, 12, 2, 41, 
	9, 2, 1, 2, 0, 1, 4, 3, 0, 2, 4, 0, 2, 1, 0, 2, 3, 1, 0, 4, 5, 46, 0, 7, 
	1, 0, 0, 0, 2, 20, 1, 0, 0, 0, 4, 29, 1, 0, 0, 0, 6, 8, 3, 2, 1, 0, 7, 
	6, 1, 0, 0, 0, 8, 9, 1, 0, 0, 0, 9, 7, 1, 0, 0, 0, 9, 10, 1, 0, 0, 0, 10, 
	1, 1, 0, 0, 0, 11, 12, 3, 4, 2, 0, 12, 13, 5, 10, 0, 0, 13, 21, 1, 0, 0, 
	0, 14, 15, 5, 8, 0, 0, 15, 16, 5, 1, 0, 0, 16, 17, 3, 4, 2, 0, 17, 18, 
	5, 10, 0, 0, 18, 21, 1, 0, 0, 0, 19, 21, 5, 10, 0, 0, 20, 11, 1, 0, 0, 
	0, 20, 14, 1, 0, 0, 0, 20, 19, 1, 0, 0, 0, 21, 3, 1, 0, 0, 0, 22, 23, 6, 
	2, -1, 0, 23, 30, 5, 9, 0, 0, 24, 30, 5, 8, 0, 0, 25, 26, 5, 6, 0, 0, 26, 
	27, 3, 4, 2, 0, 27, 28, 5, 7, 0, 0, 28, 30, 1, 0, 0, 0, 29, 22, 1, 0, 0, 
	0, 29, 24, 1, 0, 0, 0, 29, 25, 1, 0, 0, 0, 30, 39, 1, 0, 0, 0, 31, 32, 
	10, 5, 0, 0, 32, 33, 7, 0, 0, 0, 33, 38, 3, 4, 2, 6, 34, 35, 10, 4, 0, 
	0, 35, 36, 7, 1, 0, 0, 36, 38, 3, 4, 2, 5, 37, 31, 1, 0, 0, 0, 37, 34, 
	1, 0, 0, 0, 38, 41, 1, 0, 0, 0, 39, 37, 1, 0, 0, 0, 39, 40, 1, 0, 0, 0, 
	40, 5, 1, 0, 0, 0, 41, 39, 1, 0, 0, 0, 5, 9, 20, 29, 37, 39
];
