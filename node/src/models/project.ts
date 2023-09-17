
import { Schema, model } from 'mongoose';

export interface IProject {
  name: string,
}

export const ProjectSchema = new Schema<IProject>({
  name: {
    type: String,
    required: true,
  }
});

export const ProjectModel = model('Project', ProjectSchema, 'projects');