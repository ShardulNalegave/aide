
import mongoose from 'mongoose';
import express from 'express';
import cors from 'cors';
import bodyParser from "body-parser";

import { tasksRoutes } from './tasks.js';
import { projectsRoutes } from './projects.js';

await mongoose.connect('mongodb://localhost:27017/aide');

const PORT: number = 5000;
const app = express();

app.use(cors());
app.use(bodyParser.json());

app.get('/', (_, res) => res.send('Aide API'))
app.use('/tasks', tasksRoutes);
app.use('/projects', projectsRoutes);

app.listen(PORT, () => console.log(`Listening on port: ${PORT}`));