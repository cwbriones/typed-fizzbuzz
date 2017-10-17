#![feature(const_fn)]
use std::marker::PhantomData;

fn main() {
    let _: Program = ();
}

trait Test {
    const val: String;
}

type Program = Map<FizzBuzzFn, Range<N16>>;

// Natural Numbers

struct Zero;
struct S<T>(PhantomData<T>);

trait Peano {
    type Succ: Peano;
}

impl Peano for Zero {
    type Succ = S<Self>;
}

impl<T> Peano for S<T> where T: Peano {
    type Succ = S<Self>;
}

type N0 = Zero;
type N1 = S<N0>;
type N2 = S<N1>;
type N3 = S<N2>;
type N4 = S<N3>;
type N5 = S<N4>;
type N16 = S<Add<Add<N5, N5>, N5>>;

trait PeanoGTE<U> { type Res: Bool; }
impl PeanoGTE<Zero> for Zero { type Res = True; }
impl<T> PeanoGTE<Zero> for S<T> { type Res = True; }
impl<T> PeanoGTE<S<T>> for Zero { type Res = False; }
impl<T, U> PeanoGTE<S<U>> for S<T> where T: PeanoGTE<U> {
    type Res = T::Res;
}

// Addition

trait Add_<U> { type Res; }

impl<T> Add_<Zero> for S<T> {
    type Res = S<T>;
}

impl<T> Add_<T> for Zero {
    type Res = T;
}

impl<T, U> Add_<S<U>> for S<T>
    where
        T: Add_<S<U>>
{
    type Res = S<T::Res>;
}

type Add<LHS, RHS> = <LHS as Add_<RHS>>::Res;

// Subtraction

trait Sub_<U> { type Res; }

impl<T> Sub_<Zero> for T
    where T: Peano,
{
    type Res = T;
}

impl<T, U> Sub_<S<U>> for S<T>
    where T: PeanoGTE<U>,
          T: Sub_<U>,
          <T as PeanoGTE<U>>::Res: BoolTrue,
{
    type Res = <T as Sub_<U>>::Res;
}

type Sub<T, U> = <T as Sub_<U>>::Res;

// Divisibility Checks

trait Div<I> {
    type Out: Bool;
}

impl<T> Div<N3> for S<T>
    where
        S<T>: Sub_<N3>,
        Sub<S<T>, N3>: Div<N3>,
{
    type Out = <Sub<S<T>, N3> as Div<N3>>::Out;
}

impl Div<N3> for Zero { type Out = True; }
impl Div<N3> for N1 { type Out = False; }
impl Div<N3> for N2 { type Out = False; }

impl<T> Div<N5> for S<T>
    where
        S<T>: Sub_<N5>,
        Sub<S<T>, N5>: Div<N5>,
{
    type Out = <Sub<S<T>, N5> as Div<N5>>::Out;
}

impl Div<N5> for Zero { type Out = True; }
impl Div<N5> for N1 { type Out = False; }
impl Div<N5> for N2 { type Out = False; }
impl Div<N5> for N3 { type Out = False; }
impl Div<N5> for N4 { type Out = False; }

// Boolean Algebra

struct True;
struct False;

trait Bool {}
impl Bool for True {}
impl Bool for False {}

trait BoolTrue {}
trait BoolFalse {}

impl BoolTrue for True {}
impl BoolFalse for False {}

// Lists

struct Nil;
struct Cons<X, Xs>(PhantomData<X>, PhantomData<Xs>);

trait List {}
impl List for Nil {}
impl<X, Xs: List> List for Cons<X, Xs> {}

trait ReverseRec<Acc> {
    type Out;
}

// Reverse(Nil, Acc) -> Acc
impl<Acc> ReverseRec<Acc> for Nil {
    type Out = Acc;
}

// Reverse(X:Xs, Acc) -> Reverse(Xs, X:Acc)
impl<X, Xs, Acc> ReverseRec<Acc> for Cons<X, Xs>
    where
        Xs: ReverseRec<Cons<X, Acc>>
{
    type Out = <Xs as ReverseRec<Cons<X, Acc>>>::Out;
}

type Reverse<Xs> = <Xs as ReverseRec<Nil>>::Out;

trait Range_ { type Res: List; }
impl Range_ for Zero { type Res = Nil; }
impl<T> Range_ for S<T>
    where T: Range_ + Peano
{
    type Res = Cons<T, <T as Range_>::Res>;
}

type Range<N> = Reverse<<N as Range_>::Res>;

trait Apply<T> { type Res; }

trait Map_<F> { type Res; }
impl<F> Map_<F> for Nil { type Res = Nil; }
impl<F, X, Xs> Map_<F> for Cons<X, Xs>
    where F: Apply<X>,
          Xs: Map_<F>
{ type Res = Cons<<F as Apply<X>>::Res, <Xs as Map_<F>>::Res>; }

type Map<F, Xs> = <Xs as Map_<F>>::Res;

// Fizz Buzz!

struct Fizz;
struct Buzz;
struct FizzBuzz;
struct Say<T>(PhantomData<T>);

trait FizzBuzz_ {
    type Out;
}

impl<T: Peano> FizzBuzz_ for T
    where
        T: Div<N3>,
        T: Div<N5>,
        (<T as Div<N3>>::Out, <T as Div<N5>>::Out, T): FizzBuzz_
{
    type Out = <(<T as Div<N3>>::Out, <T as Div<N5>>::Out, T) as FizzBuzz_>::Out;
}

impl<T: Peano> FizzBuzz_ for (True, False, T) {
    type Out = Fizz;
}

impl<T: Peano> FizzBuzz_ for (False, True, T) {
    type Out = Buzz;
}

impl<T: Peano> FizzBuzz_ for (False, False, T) {
    type Out = Say<T>;
}

impl<T: Peano> FizzBuzz_ for (True, True, T) {
    type Out = FizzBuzz;
}

struct FizzBuzzFn;

impl<T> Apply<T> for FizzBuzzFn
    where
        T: Peano,
        T: Div<N3>,
        T: Div<N5>,
        (<T as Div<N3>>::Out, <T as Div<N5>>::Out, T): FizzBuzz_
{
    type Res = <T as FizzBuzz_>::Out;
}

