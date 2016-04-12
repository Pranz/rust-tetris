use std::hash::Hash;
use std::collections::HashMap;

pub struct PairMap<A: Copy + Hash,B: Copy + Hash>(HashMap<A,B>,HashMap<B,A>);
impl<A,B> PairMap<A,B>
	where A: Copy + Hash + Eq,
	      B: Copy + Hash + Eq
{
	pub fn new() -> Self{PairMap(HashMap::new(),HashMap::new())}
}

/*pub trait Map<A,B>{
	fn has(&self,key: A) -> bool{self.get(key).is_some()}
	fn get(&self,key: A) -> Option<B>;
	fn get_mut(&mut self,key: A) -> Option<B>;
	fn insert(&mut self,key: A,value: B) -> Option<B>;
	fn remove(&mut self,key: A) -> Option<B>;
	fn clear(&mut self);
	fn is_empty(&self) -> bool;
	fn len(&self) -> usize;
}*/

impl<A,B> /*Map<A,B> for*/ PairMap<A,B>
	where A: Copy + Hash + Eq,
	      B: Copy + Hash + Eq,
	      /*A != B*/
{
	pub fn get(&self,key: A) -> Option<B>{self.0.get(&key).map(|&v| v)}
	pub fn get_mut(&mut self,key: A) -> Option<&mut B>{self.0.get_mut(&key)}
	pub fn insert(&mut self,key: A,value: B) -> Option<B>{
		match self.0.insert(key,value){
			Some(old_value) => {
				self.1.remove(&old_value);
				self.1.insert(value,key);
				Some(old_value)
			},
			None => None
		}
	}
	pub fn remove(&mut self,key: A) -> Option<B>{
		if let Some(value) = self.0.remove(&key){
			self.1.remove(&value);
			Some(value)
		}else{
			None
		}
	}
	pub fn clear(&mut self){
		self.0.clear();
		self.1.clear();
	}
	pub fn is_empty(&self) -> bool{
		self.0.is_empty()
	}
	pub fn len(&self) -> usize{
		self.0.len()
	}
/*}

impl<A,B> Map<B,A> for PairMap<A,B>
	where A: Copy + Hash,
	      B: Copy + Hash,
	      A != B
{*/
	pub fn get2(&self,key: B) -> Option<A>{self.1.get(&key).map(|&v| v)}
	pub fn get_mut2(&mut self,key: B) -> Option<&mut A>{self.1.get_mut(&key)}
	pub fn insert2(&mut self,key: B,value: A) -> Option<A>{
		match self.1.insert(key,value){
			Some(old_value) => {
				self.0.remove(&old_value);
				self.0.insert(value,key);
				Some(old_value)
			},
			None => None
		}
	}
	pub fn remove2(&mut self,key: B) -> Option<A>{
		if let Some(value) = self.1.remove(&key){
			self.0.remove(&value);
			Some(value)
		}else{
			None
		}
	}
	pub fn clear2(&mut self){
		self.0.clear();
		self.1.clear();
	}
	pub fn is_empty2(&self) -> bool{
		self.1.is_empty()
	}
	pub fn len2(&self) -> usize{
		self.1.len()
	}
}
