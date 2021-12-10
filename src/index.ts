import { Command } from 'commander';
import path from 'path';
import pkg from '../package.json';
import { buklImport } from './common/utils';

const program = new Command();

program.version(pkg.version);

buklImport(path.join(__dirname, './modules')).map((command) => {
  program.command(command.optionsStr).description(command.description).action(command.action);
});

program.parse();
