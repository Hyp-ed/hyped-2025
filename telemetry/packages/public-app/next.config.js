/** @type {import('next').NextConfig} */
const nextConfig = {
	webpack: (config, context) => {
		// Enable polling based on env variable being set
		if (process.env.NEXT_WATCH) {
			config.watchOptions = {
				poll: 500,
				aggregateTimeout: 300,
			};
		}
		return config;
	},
};

module.exports = nextConfig;
// next.config.js
