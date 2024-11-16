/**
 * OpenMCT view. Creates the `iframe` for loading the OpenMCT dashboard.
 */
export const OpenMCT = () => (
	<iframe
		id="openmct"
		className="h-full w-full rounded-xl"
		src="../../openmct/index.html"
	></iframe>
);
