#!/usr/bin/env python3

import inspect

def outer_locals(depth=0):
    """
    With depth=0 behaves like locals(), depth=1 are the locals of the
    containing frame, depth=2 the locals of the frame containing the containing
    frame and so on...
    """
    return inspect.getouterframes(inspect.currentframe())[depth+1][0].f_locals

def lamb(*args, **kwargs):
	#for key, val in kwargs:
	keys = kwargs.keys()
	return lambda *keys: [print(keys[0])]
	#eval(compile("\n".join(args), '<string>', 'eval'))
def do(*args):
	exec(compile("\n".join(args), '<string>', 'exec'))
	preamble = ['loc = outer_locals(depth=1)'
	            'for name,val in loc']
	for name in args:
		preamble.append(name+'=loc['+name+']')
	preamble.append(args)

def lamb(*exprs):
	if len(exprs) < 1:
		raise TypeError('must contain at least 1 expression')
	if isinstance(exprs[0], tuple):
		args = exprs[0]
		exprs = exprs[1:]
	else:
		args = []
	code = compile("\n".join(exprs), '<string>', 'eval')
	return lambda *a: exec(code, globals(), outer_locals())

def λ(*exprs):
	if len(exprs) < 1:
		raise TypeError("need at least one expression")
	if isinstance(exprs[0], tuple):
		if len(exprs) < 2:
			raise TypeError("need at least one expression")
		code = ['def __function__('+','.join(exprs[0])+'):']
		code.extend(exprs[1:])
	else:
		code = ['def __function__():']
		code.extend(exprs)
	code, d = "\n    ".join(code), {}
	exec(code, globals(), d)
	return loc['__function__']

def chain(*iterables):
	for i in iterables:
		yield from i
		
def λ(*exprs):
	if isinstance(exprs[0], tuple):
		code = chain(('def f('+','.join(exprs[0])+'):',), exprs[1:])
	else:
		code = chain(('def f():',), exprs)
	code = "\n    ".join(code)
	d = {}
	exec(code, globals(), d)
	return d['f']

def do(*exprs):
	exec("\n".join(exprs), globals(), inspect.getouterframes(inspect.currentframe())[1][0].f_locals)

def main():
	print('---lambda tests---')
	print(λ(('a','b'), 'r = [None, None]',
	                   'r[0] = a*b',
	                   'r[1] = a+b',
	                   'return r')(7,8))
	print(λ(('a','b'), 'r = [a, b]',
	                   'r[0], r[1] = r[0]*r[1], r[0]+r[1]',
	                   'return r')(9,3))
	print(λ('r = [None, None]',
	        'r[0] = 3*4',
	        'r[1] = 3+4',
	        'return r')())
	print('---lambda list reversal---')
	a = [1,2,3,4,5]
	λ(('L',), 'for i in range(len(L)//2):','\tL[i], L[-i-1] = L[-i-1], L[i]')(a)
	print(a)
	print('---do---')
	a = [1,2,3,4,5]
	do ('for i in range(len(a)//2):',
		'    a[i], a[-i-1] = a[-i-1], a[i]')
	print(a)

	def foo():
		print('bar')
		a = [0,1]
		a[0] = 3*4
		print(a)
		return exec('print("message")\nprint(a)\na[1]=a[0]*7',globals(),locals())
	print([foo(), '<-- foo'])

if __name__ == '__main__':
	main()