import { Command } from 'commander';
import { registry } from './modules';

const program = new Command();

program.command(registry.optionsStr).description(registry.description).action(registry.action);

program.parse();
