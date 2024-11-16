import type { SVGProps } from 'react';

/**
 * The HYPED logo as an SVG.
 * @param props (Optional) props to pass to the SVG.
 * @returns An SVG component.
 */
export const Logo = (props: SVGProps<SVGSVGElement>) => (
	<svg
		id="Layer_1"
		xmlns="http://www.w3.org/2000/svg"
		x={0}
		y={0}
		viewBox="0 0 841.89 400"
		xmlSpace="preserve"
		{...props}
	>
		<style>{'.st1{fill:#ae2025}'}</style>
		<path
			d="M85.42 230.23v135.44H54.74V230.23h30.68zm50.81 0v51.84H92.66v28.31h43.56v55.28h30.67V230.23h-30.66zm99.48 0h-30.78l48.47 74.87v60.57h30.67v-60.59l-48.36-74.85zm62.33 0-26.9 41.49 17.25 26.71 44.16-68.19h-34.51zM474 271.5c0 26.66-18.4 42.59-49.2 42.59h-16.3v-27.76h10.75c20.69 0 24.33-4.87 24.33-15.09 0-6.87-2.42-13.22-20.09-13.22h-22.23v107.66h-30.67V230.23h52.11c32.59 0 51.3 15.05 51.3 41.27zm312.66 24.35c0 45.04-26.6 69.83-74.87 69.83h-6.24v-27.76h.42c35.98 0 49.99-11.79 49.99-42.06 0-18.71-5.02-37.83-42.33-37.83H698.3v107.66h-30.69V230.23h48.14c45.72 0 70.91 23.32 70.91 65.62z"
			style={{
				fill: '#fff',
			}}
		/>
		<path
			className="st1"
			d="M515.92 265.36v-35.12h92.05l17.89 35.12H515.92zM515.75 365.68v-35.12h92.05l17.89 35.12H515.75zM515.92 315.56v-35.12h92.05l17.89 35.12H515.92z"
		/>
	</svg>
);
