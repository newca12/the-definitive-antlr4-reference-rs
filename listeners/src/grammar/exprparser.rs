// Generated from Expr.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::exprlistener::*;
use super::exprvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const MULT:isize=1; 
		pub const ADD:isize=2; 
		pub const INT:isize=3; 
		pub const WS:isize=4;
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
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
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
	for<'x> Visitable<dyn ExprVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=ExprParserContextType>
{}

antlr_rust::coerce_from!{ 'input : ExprParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn ExprParserContext<'input> + 'input
where
    T: ExprVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn ExprVisitor<'input> + 'x))
    }
}

impl<'input> ExprParserContext<'input> for TerminalNode<'input,ExprParserContextType> {}
impl<'input> ExprParserContext<'input> for ErrorNode<'input,ExprParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn ExprParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn ExprListener<'input> + 'input }

pub struct ExprParserContextType;
antlr_rust::tid!{ExprParserContextType}

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
antlr_rust::tid! { ExprParserExt<'a> }

impl<'input> TokenAware<'input> for ExprParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for ExprParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for ExprParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "Expr.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn ExprParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					1 => ExprParser::<'input,I,_>::e_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> ExprParser<'input, I, DefaultErrorStrategy<'input,ExprParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn e_sempred(_localctx: Option<&EContext<'input>>, pred_index:isize,
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

impl<'input> ExprParserContext<'input> for SContext<'input>{}

impl<'input,'a> Listenable<dyn ExprListener<'input> + 'a> for SContext<'input>{
		fn enter(&self,listener: &mut (dyn ExprListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_s(self);
		}
		fn exit(&self,listener: &mut (dyn ExprListener<'input> + 'a)) {
			listener.exit_s(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ExprVisitor<'input> + 'a> for SContext<'input>{
	fn accept(&self,visitor: &mut (dyn ExprVisitor<'input> + 'a)) {
		visitor.visit_s(self);
	}
}

impl<'input> CustomRuleContext<'input> for SContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_s }
	//fn type_rule_index() -> usize where Self: Sized { RULE_s }
}
antlr_rust::tid!{SContextExt<'a>}

impl<'input> SContextExt<'input>{
	fn new(parent: Option<Rc<dyn ExprParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SContextAttrs<'input>: ExprParserContext<'input> + BorrowMut<SContextExt<'input>>{

fn e(&self) -> Option<Rc<EContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SContextAttrs<'input> for SContext<'input>{}

impl<'input, I, H> ExprParser<'input, I, H>
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
pub type EContextAll<'input> = EContext<'input>;


pub type EContext<'input> = BaseParserRuleContext<'input,EContextExt<'input>>;

#[derive(Clone)]
pub struct EContextExt<'input>{
	pub op: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> ExprParserContext<'input> for EContext<'input>{}

impl<'input,'a> Listenable<dyn ExprListener<'input> + 'a> for EContext<'input>{
		fn enter(&self,listener: &mut (dyn ExprListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_e(self);
		}
		fn exit(&self,listener: &mut (dyn ExprListener<'input> + 'a)) {
			listener.exit_e(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn ExprVisitor<'input> + 'a> for EContext<'input>{
	fn accept(&self,visitor: &mut (dyn ExprVisitor<'input> + 'a)) {
		visitor.visit_e(self);
	}
}

impl<'input> CustomRuleContext<'input> for EContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ExprParserContextType;
	fn get_rule_index(&self) -> usize { RULE_e }
	//fn type_rule_index() -> usize where Self: Sized { RULE_e }
}
antlr_rust::tid!{EContextExt<'a>}

impl<'input> EContextExt<'input>{
	fn new(parent: Option<Rc<dyn ExprParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EContextExt{
				op: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait EContextAttrs<'input>: ExprParserContext<'input> + BorrowMut<EContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,ExprParserContextType>>> where Self:Sized{
	self.get_token(INT, 0)
}
fn e_all(&self) ->  Vec<Rc<EContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn e(&self, i: usize) -> Option<Rc<EContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token MULT
/// Returns `None` if there is no child corresponding to token MULT
fn MULT(&self) -> Option<Rc<TerminalNode<'input,ExprParserContextType>>> where Self:Sized{
	self.get_token(MULT, 0)
}
/// Retrieves first TerminalNode corresponding to token ADD
/// Returns `None` if there is no child corresponding to token ADD
fn ADD(&self) -> Option<Rc<TerminalNode<'input,ExprParserContextType>>> where Self:Sized{
	self.get_token(ADD, 0)
}

}

impl<'input> EContextAttrs<'input> for EContext<'input>{}

impl<'input, I, H> ExprParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  e(&mut self,)
	-> Result<Rc<EContextAll<'input>>,ANTLRError> {
		self.e_rec(0)
	}

	fn e_rec(&mut self, _p: isize)
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
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			recog.base.set_state(7);
			recog.base.match_token(INT,&mut recog.err_handler)?;

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
							/*recRuleAltStartAction*/
							let mut tmp = EContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_e);
							_localctx = tmp;
							recog.base.set_state(9);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(10);
							let tmp = recog.base.match_token(MULT,&mut recog.err_handler)?;
							 cast_mut::<_,EContext >(&mut _localctx).op = Some(tmp.clone());
							  

							/*InvokeRule e*/
							recog.base.set_state(11);
							recog.e_rec(4)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = EContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_e);
							_localctx = tmp;
							recog.base.set_state(12);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(13);
							let tmp = recog.base.match_token(ADD,&mut recog.err_handler)?;
							 cast_mut::<_,EContext >(&mut _localctx).op = Some(tmp.clone());
							  

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

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x06\x17\x04\x02\x09\x02\x04\x03\x09\x03\x03\x02\x03\x02\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x07\x03\x12\x0a\
	\x03\x0c\x03\x0e\x03\x15\x0b\x03\x03\x03\x02\x03\x04\x04\x02\x04\x02\x02\
	\x02\x16\x02\x06\x03\x02\x02\x02\x04\x08\x03\x02\x02\x02\x06\x07\x05\x04\
	\x03\x02\x07\x03\x03\x02\x02\x02\x08\x09\x08\x03\x01\x02\x09\x0a\x07\x05\
	\x02\x02\x0a\x13\x03\x02\x02\x02\x0b\x0c\x0c\x05\x02\x02\x0c\x0d\x07\x03\
	\x02\x02\x0d\x12\x05\x04\x03\x06\x0e\x0f\x0c\x04\x02\x02\x0f\x10\x07\x04\
	\x02\x02\x10\x12\x05\x04\x03\x05\x11\x0b\x03\x02\x02\x02\x11\x0e\x03\x02\
	\x02\x02\x12\x15\x03\x02\x02\x02\x13\x11\x03\x02\x02\x02\x13\x14\x03\x02\
	\x02\x02\x14\x05\x03\x02\x02\x02\x15\x13\x03\x02\x02\x02\x04\x11\x13";

