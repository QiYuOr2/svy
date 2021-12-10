import { Command } from 'commander';
import path from 'path';
import glob from 'glob'
import pkg from '../package.json';
import { bulkImport } from './common/utils';

const program = new Command();

program.version(pkg.version);

bulkImport(glob.sync(path.join(__dirname, './modules/*.@(t|j)s'))).map((command) => {
  program.command(command.optionsStr).description(command.description).action(command.action);
});

program.parse();
