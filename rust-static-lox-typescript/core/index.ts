/* ============================================================
  ARROW FUNCTION PARSING TESTS
 ============================================================ */

type hi = Array<number>;
type Foo = {
	[Symbol.iterator](): void;
};

type Bar<T> = {
	[K in keyof T]: T[K];
};

const bar: Bar<number extends string ? string : number> = 1;

type Complex = {
	name: string;
	id?: number;
	[key: string]: any;
	greet<T>(msg: T): void;
	[name: number]: string;
};

let x = 10;
let y = 20;
let a = 1;
let b = 2;
let c = 3;

// ---------- 1. Simple single-param arrow ----------
const f1 = (x) => x + 1;

// ---------- 2. Empty param list ----------
const f2 = () => 42;

// ---------- 3. Multi-param arrow ----------
const f3 = (a, b) => a + b;

// ---------- 4. With type annotations ----------
const f4 = (a: number, b: string): string => a + b;

// ---------- 5. With optional + default params ----------
const f5 = (x?: number, y = 10) => x ?? y;

// ---------- 6. Rest params ----------
const f6 = (...args) => args.length;

// ---------- 7. Destructured parameters ----------
const f7 = ({ name, age }) => `${name}:${age}`;
const f8 = ([x, y]) => x * y;

// ---------- 8. Destructured + typed ----------
const f9 = ({ x, y }: { x: number; y: number }) => x + y;

// ---------- 9. Return type annotation ----------
const f10 = (a: string): number => a.length;

// ---------- 10. Arrow returning object literal ----------
const f11 = () => ({ ok: true });

// ---------- 11. Async arrow ----------
const f12 = async (x: number) => await Promise.resolve(x);

// ---------- 12. Generic arrow ----------
const f13 = <T>(x: T): T => x;

// ---------- 13. Async + generic ----------
const f14 = async <T>(x: T) => x;

// ---------- 14. Nested arrow ----------
const f15 = (a: number) => (b: number) => a + b;

// ---------- 15. Arrow with block body ----------
const f16 = (x: number) => {
	const y = x * 2;
	return y;
};

// ---------- 16. Arrow returning another arrow ----------
const f17 = (x: number) => (y: number) => (z: number) => x + y + z;

// ---------- 17. Arrow in higher-order call ----------
const f18 = [1, 2, 3].map((x) => x * 2);

// ---------- 18. Arrow with tuple & array types ----------
const f19 = (t: [number, string], a: number[]): void => {
	console.log(t, a);
};

// ---------- 19. Arrow returning void ----------
const f20 = (): void => {};

// ---------- 20. Arrow returning a promise ----------
const f21 = async (): Promise<number> => 10;

// ---------- 21. Arrow with type union ----------
const f22 = (x: number | string): string => x.toString();

// ---------- 22. Arrow returning conditional ----------
const f23 = (flag: boolean) => (flag ? 1 : 0);

// ---------- 23. Arrow with computed default ----------
const f24 = (x = Math.random() * 10) => x;

// ---------- 24. Parenthesized arrow (expression context) ----------
const f25 = (a: number, b: number) => a + b;

// ---------- 25. Arrow inside object ----------
const obj1 = {
	add: (a: number, b: number) => a + b,
	inc: (x) => x + 1,
};

// ---------- 26. Arrow inside array ----------
const arr = [(x: number) => x * 2, (x: string) => x.toUpperCase()];

// ---------- 27. Arrow with async block body ----------
const f26 = async (x: number) => {
	const res = await Promise.resolve(x * 2);
	return res;
};

// ---------- 28. Arrow with no params returning arrow ----------
const f27 = () => () => 123;

// ---------- 29. Arrow with type inference ----------
const f28 = (x = 10) => x * 2;

// ---------- 30. Arrow with intersection type ----------
const f29 = <T extends string & { id: number }>(x: T): string =>
	x.id.toString();

// ---------- 31. Arrow with nested generic constraint ----------
const f30 = <T extends { a: number }>(x: T): number => x.a;

// ---------- 32. Arrow returning nested object ----------
const f31 = () => ({
	meta: { ok: true, reason: "parsed" },
});

// ============================================================
//  GROUPING EXPRESSION TESTS (should NOT be arrow)
// ============================================================

// ---------- G1. Basic grouping ----------
const g1 = 1 + 2;

// ---------- G2. Nested grouping ----------
const g2 = (a + b) * 3;

// ---------- G3. Function call in grouping ----------
const g3 = f1(10);

// ---------- G4. Member access after grouping ----------
const g4 = (x + y).toString();

// ---------- G5. Grouping with comma expression ----------
const g5 = ((a = 1), (b = 2), a + b);

// ---------- G6. Grouping nested with arrow as expression ----------
const g6 = ((x) => x + 1)(10); // valid invocation

// ---------- G7. Grouping with type assertion ----------
const g7 = <number>(a + b);

// ---------- G8. Grouping with object expression ----------
const g8 = { value: 10 };

// ---------- G9. Grouping + conditional ----------
const g9 = true ? 1 : 2;

// ---------- G10. Grouping in return ----------
function wrap() {
	return 1 + 2;
}
