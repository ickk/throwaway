#!/usr/bin/env python3
import inspect, sys
from functools import wraps, partial
from collections import namedtuple

class Parser():
    def __init__(self, options=[], subparsers=[], doc=None, name=None):
        self.options = options
        self.subparsers = subparsers
        self.doc = doc or ""
        self.name = name
        self._build_match_table()
    def __getattr__(self, attr):
        return self._match_table.get(attr, None)
    def _build_match_table(self):
        self._match_table = {}
        for option in self.options:
            if option.name:
                self._match_table["--"+option.name] = option
            if option.short:
                self._match_table["-"+option.short] = option
            for alias in option.aliases:
                if len(alias)==1:
                    self._match_table["-"+alias] = option
                else:
                    self._match_table["--"+alias] = option
        for parser in self.subparsers:
            if parser.name:
                self._match_table[parser.name] = parser
    def parse(self, args=None):
        """Returns an dict with  key, vals  as  option/command names, their results
        returns list of remaining args"""
        if args is None: args = sys.argv[1:]
        ret, i = {}, 0
        while i < len(args):
            if args[i] == '--': # stop parsing
                i += 1
                break
            arg = self._match_table.get(args[i], None)
            if isinstance(arg, Option):
                i, num_params = i+1, len(inspect.signature(arg.func).parameters)
                ret[arg.name] = arg.func(*args[i:i+num_params])
                i += num_params
            elif isinstance(arg, Parser):
                ret[arg.name], args = arg.parse(args[i+1:])
                i = 0
                break
            else:
                break
        return ret, args[i:]
    @property   
    def usage_info(self):
        return "".join([self.doc, "\n"]
                       + ["\nOPTIONS:\n"]
                       + ["  "+option.summary+"\n" for option in self.options]
                       + ["\nSUBCOMMANDS:\n"]
                       + ["  "+parser.summary+"\n" for parser in self.subparsers])
    @property
    def summary(self):
        return self.name+"    "+next(iter(self.doc.splitlines()), "")

class Option():
    def __init__(self, func):
        self.func = func
    @property
    def doc(self):
        return getattr(self.func, '__doc__', "")
    @property
    def name(self):
        return getattr(self.func, '__name__', None)
    def __getattr__(self, attr):
        return getattr(self.func, attr, ())
    @property
    def summary(self):
        return '-'+str(self.short)+" --"+str(self.name)+" "+str(inspect.signature(self.func))+"    "+str(self.doc)

# Class construction interface decorators
def parser(cls):
    options = [Option(func) for _, func in inspect.getmembers(cls, predicate=inspect.isfunction)]
    subparsers = [parser for _, parser in inspect.getmembers(cls, predicate=lambda x: isinstance(x, Parser))]
    return Parser(options, subparsers, doc=cls.__doc__, name=cls.__name__)

def option(_func=None, **kwargs):
    if _func is None:
        return partial(option, **kwargs)
    _func.__dict__.update(kwargs)
    _func = staticmethod(_func) # This must come after dict update.
    return _func

if __name__ == '__main__':
    # Example cli parser declaration
    @parser
    class cli():
        """Proof of concept command line parser v0.0.2"""
        @option(short='hw', aliases=('hello','world'))
        def hello_world():
            """This option says 'hello world'"""
            print("hello, world!")

        @option(short='f')
        def func(arg="henlo"):
            """This option does func"""
            return [arg]

        @option(short='h')
        def help():
            """Print this message"""
            print(cli.usage_info)

        @parser
        class info():
            """Docstring for the 'info' subcommand
            some more details about the info subcommand."""
            @option(short='h')
            def help():
                """Print this message"""
                print(cli.info.usage_info)

        @parser
        class second:
            @option(short='h')
            def help():
                """Print this message"""
                print(cli.second.usage_info)
            @parser
            class third:
                """Docstring for the 'subsub' subsubcommand"""
                @option(short='h')
                def help():
                    """Print this message"""
                    print(cli.second.third.usage_info)
                @parser
                class fourth:
                    @option(short='h')
                    def help():
                        """Print this message"""
                        print(cli.second.third.fourth.usage_info)

    cli, args = cli.parse()
    if 'info' in cli:
        print('** Run with `info` **')
        if 'subsub' in cli['info']:
            print('** Run with `subsub` **')
    if 'func' in cli:
        print(cli)
    print('obj:      ', cli)
    print('left-over:', args)
