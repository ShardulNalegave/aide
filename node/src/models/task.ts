
import { Schema, model } from 'mongoose';

export interface ITask {
  title: string,
  description: string,
  completed: boolean,
  project: string,
}

export const TaskSchema = new Schema<ITask>({
  title: {
    type: String,
    required: true,
  },
  description: {
    type: String,
    default: '',
  },
  completed: {
    type: Boolean,
    default: false,
  },
  project: {
    type: String,
    required: true,
  },
});

export const TaskModel = model('Task', TaskSchema, 'tasks');