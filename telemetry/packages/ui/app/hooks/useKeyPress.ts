import { useCallback, useEffect, useLayoutEffect, useRef } from 'react';

type Key = string;
type Combo = 'ctrlKey' | 'shiftKey' | 'altKey' | 'metaKey';
type KeyCombination = [Combo, Key];

/**
 * Adds event listeners for the given key(s) and calls the callback when they are pressed.
 * Also supports key combinations.
 * @param keys The keys to listen for.
 */
export const useKeyPress = (
	keys: (Key | KeyCombination)[],
	callback: (event: KeyboardEvent) => void,
	node = window,
) => {
	const callbackRef = useRef(callback);
	useLayoutEffect(() => {
		callbackRef.current = callback;
	});

	// handle what happens on key press
	const handleKeyPress = useCallback(
		(event: KeyboardEvent) => {
			if (keys.includes(event.key)) {
				callback(event);
				return;
			}
			const KeyCombinations = keys.filter(isKeyCombination);
			for (const keyCombination of KeyCombinations) {
				const [combinationKey, key] = keyCombination;
				if (event[combinationKey] && event.key === key) {
					callback(event);
					return;
				}
			}
		},
		[keys, callback],
	);

	useEffect(() => {
		node.addEventListener('keydown', handleKeyPress);
		return () => {
			node.removeEventListener('keydown', handleKeyPress);
		};
	}, [handleKeyPress, node]);
};

/**
 * @returns `true` if the given key is a key combination, `false` otherwise.
 */
const isKeyCombination = (key: Key | KeyCombination): key is KeyCombination =>
	Array.isArray(key);
