
import { Container, MantineProvider, ScrollArea } from '@mantine/core';
import { useGET } from './hooks/loadData';
import { IProject } from './models/project';
import { ProjectList } from './components/projectList';

export default function App() {
  const { data: projectsData, loaded: projectsLoaded } = useGET<IProject[]>('http://localhost:5000/projects', []);

  return (
    <MantineProvider withNormalizeCSS withGlobalStyles theme={{ colorScheme: 'dark' }}>
      <ScrollArea style={{ background: '#141517', color: '#EFEFEF', height: '100vh' }}>
        {
          !projectsLoaded ?
            <></>
            :
            <Container>
              <ProjectList projects={projectsData} />
            </Container>
        }
      </ScrollArea>
    </MantineProvider>
  );
}