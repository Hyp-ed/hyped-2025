export function convertNamespaceToPodId(namespace: string) {
	const possiblePodId = namespace.split('.')[1];

	if (!possiblePodId) {
		throw new Error(`Invalid namespace: ${namespace}`);
	}

	return possiblePodId;
}
