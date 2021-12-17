import path from 'path';
import fs from 'fs-extra';
import chalk from 'chalk';
import { createAction, transferName } from '../common/utils';

const template = (name: string) => `
import { createAction } from '../common/utils';

export const ${transferName(name)} = createAction({
  name: '${name}',
  args: { name: { type: String } },
  description: '创建新的命令',
  action({ name }) {},
});
`;

export const create = createAction({
  name: 'create',
  args: { name: { type: String, required: true } },
  description: '创建新的命令',
  action({ name }) {
    const filename = path.join(process.cwd(), 'src/modules', `${name}.ts`);
    console.log(filename);
    if (fs.existsSync(filename)) {
      console.log(chalk.red(`${name}.ts 已存在！`));
      return;
    }
    fs.writeFileSync(filename, template(name));
  },
});
