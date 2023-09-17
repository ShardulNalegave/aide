
import axios from "axios";
import { useState } from "react";
import { Button, Container, ScrollArea, TextInput } from "@mantine/core";
import { ProjectList } from "../components/projectList";
import { useGET } from "../hooks/loadData";
import { IProject } from "../models/project";

export function HomePage() {
  const { data: projectsData, loaded: projectsLoaded, reload } = useGET<IProject[]>('http://localhost:5000/projects', []);
  const [ projectName, setProjectName ] = useState('');

  const createProject = async () => {
    await axios.post('http://localhost:5000/projects', {
      name: projectName.trim(),
    });
    reload();
  };

  return (
    <ScrollArea style={{ background: '#141517', color: '#EFEFEF', height: '100vh' }}>
      {
        !projectsLoaded ?
          <></>
          :
          <Container style={{ paddingTop: '30px' }}>
            <div>
              <TextInput label='Project Name' placeholder='Test project ABC...' value={projectName} onChange={(e) => {
                setProjectName(e.target.value);
              }}></TextInput>
              <div style={{ height: '15px' }}></div>
              <Button onClick={createProject}>Create Project</Button>
            </div>
            <div style={{ height: '25px' }}></div>
            <ProjectList projects={projectsData} />
          </Container>
      }
    </ScrollArea>
  );
}