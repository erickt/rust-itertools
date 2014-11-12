var searchIndex = {};
searchIndex['itertools'] = {"items":[[0,"","itertools","Itertools — extra iterator adaptors, functions and macros"],[1,"Clones","","An iterator like `.map(|elt| elt.clone())`"],[1,"Interleave","","Alternate elements from two iterators until both\nare run out"],[1,"Product","","An iterator adaptor that iterates over the cartesian product of\nthe element sets of two iterators `I` and `J`."],[1,"PutBack","","An iterator adaptor that allows putting back a single\nitem to the front of the iterator."],[1,"FnMap","","Clonable iterator adaptor to map elementwise\nfrom `Iterator<A>` to `Iterator<B>`"],[1,"BoxIter","","A wrapper around `Box<Iterator<A>>` so that it implements the\nIterator trait."],[11,"iter","","The wrapped iterator pointer",0],[1,"Intersperse","","An iterator adaptor to insert a particular value\nbetween each element of the adapted iterator."],[1,"Stride","","Similar to the slice iterator, but with a certain number of steps\n(stride) skipped per iteration."],[1,"StrideMut","","Stride with mutable elements"],[1,"Times","","Iterator to repeat a simple number of times"],[1,"FlatTuples","",""],[11,"iter","","",1],[3,"times","","Return an iterator with `n` elements, for simple repetition\na particular number of times. The iterator yields a counter."],[3,"write","","Assign to each reference in `to` from `from`, stopping\nat the shortest of the two iterators."],[10,"new","","",2],[10,"next","","",2],[10,"size_hint","","",2],[10,"next_back","","",2],[10,"clone","","",3],[10,"new","","",3],[10,"next","","",3],[10,"new","","",4],[10,"next","","",4],[10,"size_hint","","",4],[10,"next_back","","",4],[10,"clone","","",4],[10,"clone","","",5],[10,"new","","Iterator element type is `A`",5],[10,"put_back","","Put back a single value to the front of the iterator.",5],[10,"next","","",5],[10,"size_hint","","",5],[10,"clone","","",6],[10,"new","","Create a new cartesian product iterator",6],[10,"next","","",6],[10,"size_hint","","",6],[10,"from_iter","","Create a BoxIter from an iterator value",0],[10,"from_box","","Create a BoxIter from an already boxed iterator",0],[10,"next","","",0],[10,"size_hint","","",0],[10,"clone","","",7],[10,"new","","Create a new Intersperse iterator",7],[10,"next","","",7],[10,"size_hint","","",7],[10,"from_ptr_len","","Create Stride iterator from a raw pointer.",8],[10,"from_ptr_len","","Create Stride iterator from a raw pointer.",9],[10,"from_slice","","Create Stride iterator from a slice and the element step count.",8],[10,"from_stride","","Create Stride iterator from an existing Stride iterator",8],[10,"swap_ends","","Swap the begin and end and reverse the stride,\nin effect reversing the iterator.",8],[10,"next","","",8],[10,"size_hint","","",8],[10,"next_back","","",8],[10,"index","","",8],[10,"fmt","","",8],[10,"from_slice","","Create Stride iterator from a slice and the element step count.",9],[10,"from_stride","","Create Stride iterator from an existing Stride iterator",9],[10,"swap_ends","","Swap the begin and end and reverse the stride,\nin effect reversing the iterator.",9],[10,"next","","",9],[10,"size_hint","","",9],[10,"next_back","","",9],[10,"index","","",9],[10,"fmt","","",9],[10,"clone","","",8],[10,"index_mut","","",9],[10,"clone","","",10],[10,"next","","",10],[10,"size_hint","","",10],[10,"next_back","","",10],[6,"AppendTuple","","A helper trait for (x,y,z) ++ w => (x,y,z,w),\nused for implementing `iproduct!` and `izip!`"],[9,"append","","",11],[6,"Itertools","","Extra iterator methods for arbitrary iterators"],[10,"fn_map","","Like regular `.map`, but using a simple function pointer instead,\nso that the resulting `FnMap` iterator value can be cloned.",12],[10,"interleave","","Alternate elements from two iterators until both\nare run out",12],[10,"intersperse","","An iterator adaptor to insert a particular value\nbetween each element of the adapted iterator.",12],[10,"dropn","","Consume `n` elements of the iterator eagerly",12],[10,"drain","","Run the iterator, eagerly, to the end and consume all its elements.",12],[10,"apply","","Run the closure `f` eagerly on each element of the iterator.",12],[6,"ItertoolsClonable","",""],[10,"clones","","An iterator like `.map(|elt| elt.clone())`",13],[10,"clone","","",1],[10,"next","","",1],[10,"size_hint","","",1],[10,"next_back","","",1],[15,"iproduct!","","Create an iterator over the “cartesian product” of iterators."],[15,"izip!","","Create an iterator running multiple iterators in lockstep."],[15,"icompr!","","`icompr` as in “iterator comprehension” allows creating a\nmapped iterator with simple syntax, similar to set builder notation,\nand directly inspired by Python. Supports an optional filter clause.\n \nSyntax:\n \n `icompr!(<expression> for <pattern> in <iterator>)`"]],"paths":[[1,"BoxIter"],[1,"FlatTuples"],[1,"Clones"],[1,"Interleave"],[1,"FnMap"],[1,"PutBack"],[1,"Product"],[1,"Intersperse"],[1,"Stride"],[1,"StrideMut"],[1,"Times"],[6,"AppendTuple"],[6,"Itertools"],[6,"ItertoolsClonable"]]};
initSearch(searchIndex);