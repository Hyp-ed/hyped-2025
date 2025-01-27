import * as fs from 'node:fs';
import * as path from 'node:path';
import * as YAML from 'yaml';

const podsYaml = path.join(
	__dirname,
	'..',
	'..',
	'..',
	'..',
	'..',
	'..',
	'config',
	'pods.yaml',
);

const yamlContent = fs.readFileSync(podsYaml, 'utf8');

const parsedYaml = YAML.parse(yamlContent);

const pods = Object.entries(parsedYaml.pods).reduce(
	(result: { [key: string]: any }, [podId, podData]) => {
		const pod = podData as {
			name: string;
			measurements: { [key: string]: any };
		};
		result[podId] = {
			id: podId,
			name: pod.name,
			measurements: Object.entries(pod.measurements).reduce(
				(measResult, [key, measData]) => ({
					...measResult,
					[key]: {
						...(measData as object),
						key,
					},
				}),
				{},
			),
		};
		return result;
	},
	{},
);

console.log(pods);
console.log(pods.pod_1.measurements.keyence);
