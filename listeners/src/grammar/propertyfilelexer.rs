// Generated from PropertyFile.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use antlr_rust::atn::ATN;
use antlr_rust::char_stream::CharStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::TokenSource;
use antlr_rust::token_factory::{TokenFactory,CommonTokenFactory,TokenAware};
use antlr_rust::token::*;
use antlr_rust::rule_context::{BaseRuleContext,EmptyCustomRuleContext,EmptyContext};
use antlr_rust::parser_rule_context::{ParserRuleContext,BaseParserRuleContext,cast};
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};

use antlr_rust::{lazy_static,Tid,TidAble,TidExt};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};


	pub const T__0:isize=1; 
	pub const T__1:isize=2; 
	pub const ID:isize=3; 
	pub const STRING:isize=4;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;4] = [
		"T__0", "T__1", "ID", "STRING"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;3] = [
		None, Some("'='"), Some("'\n'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;5]  = [
		None, None, None, Some("ID"), Some("STRING")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub type LexerContext<'input> = BaseRuleContext<'input,EmptyCustomRuleContext<'input,LocalTokenFactory<'input> >>;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a> >::From;

pub struct PropertyFileLexer<'input, Input:CharStream<From<'input> >> {
	base: BaseLexer<'input,PropertyFileLexerActions,Input,LocalTokenFactory<'input>>,
}

antlr_rust::tid! { impl<'input,Input> TidAble<'input> for PropertyFileLexer<'input,Input> where Input:CharStream<From<'input> > }

impl<'input, Input:CharStream<From<'input> >> Deref for PropertyFileLexer<'input,Input>{
	type Target = BaseLexer<'input,PropertyFileLexerActions,Input,LocalTokenFactory<'input>>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl<'input, Input:CharStream<From<'input> >> DerefMut for PropertyFileLexer<'input,Input>{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl<'input, Input:CharStream<From<'input> >> PropertyFileLexer<'input,Input>{
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "PropertyFileLexer.g4"
    }

	pub fn new_with_token_factory(input: Input, tf: &'input LocalTokenFactory<'input>) -> Self {
		antlr_rust::recognizer::check_version("0","3");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				PropertyFileLexerActions{},
				tf
			)
	    }
	}
}

impl<'input, Input:CharStream<From<'input> >> PropertyFileLexer<'input,Input> where &'input LocalTokenFactory<'input>:Default{
	pub fn new(input: Input) -> Self{
		PropertyFileLexer::new_with_token_factory(input, <&LocalTokenFactory<'input> as Default>::default())
	}
}

pub struct PropertyFileLexerActions {
}

impl PropertyFileLexerActions{
}

impl<'input, Input:CharStream<From<'input> >> Actions<'input,BaseLexer<'input,PropertyFileLexerActions,Input,LocalTokenFactory<'input>>> for PropertyFileLexerActions{
	}

	impl<'input, Input:CharStream<From<'input> >> PropertyFileLexer<'input,Input>{

}

impl<'input, Input:CharStream<From<'input> >> LexerRecog<'input,BaseLexer<'input,PropertyFileLexerActions,Input,LocalTokenFactory<'input>>> for PropertyFileLexerActions{
}
impl<'input> TokenAware<'input> for PropertyFileLexerActions{
	type TF = LocalTokenFactory<'input>;
}

impl<'input, Input:CharStream<From<'input> >> TokenSource<'input> for PropertyFileLexer<'input,Input>{
	type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn IntStream> {
        self.base.get_input_stream()
    }

	fn get_source_name(&self) -> String {
		self.base.get_source_name()
	}

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
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
		"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x06\x1d\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x03\x02\x03\x02\x03\x03\x03\x03\x03\x04\x06\x04\x11\x0a\x04\x0d\
		\x04\x0e\x04\x12\x03\x05\x03\x05\x07\x05\x17\x0a\x05\x0c\x05\x0e\x05\x1a\
		\x0b\x05\x03\x05\x03\x05\x03\x18\x02\x06\x03\x03\x05\x04\x07\x05\x09\x06\
		\x03\x02\x03\x03\x02\x63\x7c\x02\x1e\x02\x03\x03\x02\x02\x02\x02\x05\x03\
		\x02\x02\x02\x02\x07\x03\x02\x02\x02\x02\x09\x03\x02\x02\x02\x03\x0b\x03\
		\x02\x02\x02\x05\x0d\x03\x02\x02\x02\x07\x10\x03\x02\x02\x02\x09\x14\x03\
		\x02\x02\x02\x0b\x0c\x07\x3f\x02\x02\x0c\x04\x03\x02\x02\x02\x0d\x0e\x07\
		\x0c\x02\x02\x0e\x06\x03\x02\x02\x02\x0f\x11\x09\x02\x02\x02\x10\x0f\x03\
		\x02\x02\x02\x11\x12\x03\x02\x02\x02\x12\x10\x03\x02\x02\x02\x12\x13\x03\
		\x02\x02\x02\x13\x08\x03\x02\x02\x02\x14\x18\x07\x24\x02\x02\x15\x17\x0b\
		\x02\x02\x02\x16\x15\x03\x02\x02\x02\x17\x1a\x03\x02\x02\x02\x18\x19\x03\
		\x02\x02\x02\x18\x16\x03\x02\x02\x02\x19\x1b\x03\x02\x02\x02\x1a\x18\x03\
		\x02\x02\x02\x1b\x1c\x07\x24\x02\x02\x1c\x0a\x03\x02\x02\x02\x05\x02\x12\
		\x18\x02";
