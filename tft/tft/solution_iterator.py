import asyncio
from typing import AsyncIterator, TypeVar

# Import the SolutionRust class from your Rust module
# Replace 'tft_rust' with the actual name of your compiled Rust module if different
from tft_rust import SolutionIteratorRust, SolutionRust

T = TypeVar('T')

class AsyncSolutionIterator(AsyncIterator[SolutionRust]):
    def __init__(self, iterator: SolutionIteratorRust) -> None:
        self.iterator: SolutionIteratorRust = iterator
        self.loop: asyncio.AbstractEventLoop = asyncio.get_event_loop()

    def __aiter__(self) -> 'AsyncSolutionIterator':
        return self

    async def __anext__(self) -> SolutionRust:
        def next_solution() -> SolutionRust:
            try:
                return next(self.iterator)
            except StopIteration:
                raise StopAsyncIteration

        # Run the blocking next() call in the default executor
        try:
            result: SolutionRust = await self.loop.run_in_executor(None, next_solution)
            return result
        except StopAsyncIteration:
            raise
