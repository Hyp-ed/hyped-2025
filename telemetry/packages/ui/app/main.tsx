import '@fontsource/ibm-plex-mono';
import '@fontsource/raleway';
import * as React from 'react';
import ReactDOM from 'react-dom/client';
import { Toaster } from 'react-hot-toast';
import 'victormono';
import { App } from './App';
import './globals.css';
import { Error } from './components/error';
import { Providers } from './providers';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<Providers>
			<App />
			<Toaster
				position="bottom-center"
				reverseOrder={false}
				toastOptions={{
					className: 'bg-gray-100 text-gray-900 shadow-xl',
				}}
			/>
			<Error />
		</Providers>
	</React.StrictMode>,
);
