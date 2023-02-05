import type { AppProps } from "next/app";
import { NextUIProvider } from '@nextui-org/react';

import "../style.css";
import "../App.css";

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  return (
    <NextUIProvider>
      <Component {...pageProps} />
    </NextUIProvider>
  );
}