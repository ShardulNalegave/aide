
import { Card, Title } from "@mantine/core";
import { IProject } from "../models/project";

interface ProjectListProps {
  projects: IProject[],
}

export function ProjectList({ projects } : ProjectListProps) {
  return (
    <div>
      {projects.map(project => <Project data={project} />)}
    </div>
  );
}

interface ProjectProps {
  data: IProject,
}

export function Project({ data } : ProjectProps) {
  return (
    <Card style={{ margin: '5px' }}>
      <Title order={3}>{data.name}</Title>
    </Card>
  );
}