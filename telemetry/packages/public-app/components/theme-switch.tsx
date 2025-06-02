import { Switch } from '@tremor/react';
import { useEffect, useState } from 'react';

function ThemeSwitch() {
	const [theme, setTheme] = useState('dark');

	useEffect(() => {
		document.body.classList.remove('light', 'dark');
		document.body.classList.add(theme);
	}, [theme]);

	const [enabled, setEnabled] = useState(theme === 'dark');

	const handleThemeChange = (enabled: boolean) => {
		setTheme(enabled ? 'dark' : 'light');
		setEnabled(enabled);
	};

	return <Switch checked={enabled} onChange={handleThemeChange} />;
}

export default ThemeSwitch;
