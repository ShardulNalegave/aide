
import { MantineProvider } from '@mantine/core';
import { Outlet } from 'react-router-dom';

export default function App() {
  return (
    <MantineProvider withNormalizeCSS withGlobalStyles theme={{ colorScheme: 'dark' }}>
      <Outlet />
    </MantineProvider>
  );
}