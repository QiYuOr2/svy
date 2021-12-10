import { Command } from 'commander';
import { registry } from './modules';
import pkg from '../package.json';

const program = new Command();

program.version(pkg.version);

program.command(registry.optionsStr).description(registry.description).action(registry.action);

program.parse();
