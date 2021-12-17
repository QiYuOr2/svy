import { execSync } from 'child_process';
import chalk from 'chalk';
import { createAction, checkConfig, updateConfig, isUndefined } from '../common/utils';

const readRegistryConfig = () => {
  return checkConfig().registryList;
};

const list = () => {
  const registryList = readRegistryConfig();
  const currentRegistry = execSync('npm config get registry').toString('utf-8').trim();

  Object.keys(registryList).forEach((key) => {
    const registry = registryList[key];
    if (registry === currentRegistry) {
      console.log(chalk.green(`\n> ${key} - ${registry}`));
    } else {
      console.log(`\n- ${key} - ${registry}`);
    }
  });
};

const addRegistry = (registryList: Record<string, string>, key: string, val: string) => {
  registryList[key] = val;
  return updateConfig('registryList', registryList)[0];
};

const setRegistry = (name: string) => {
  let registryList = readRegistryConfig();

  if (name.includes('=')) {
    const [key, val] = name.split('=');
    registryList = addRegistry(registryList, key, val);
    name = key;
  }

  if (!Object.keys(registryList).includes(name)) {
    console.log(chalk.red('\n未找到对应源'));
    return;
  } else {
    try {
      execSync(`npm config set registry ${registryList[name]}`);
      console.log(`\n已切换为 ${chalk.green(name)} 源: ${registryList[name]}`);
    } catch (error) {
      console.log(chalk.red(error));
    }
  }
};

export const registry = createAction({
  name: 'registry',
  args: {
    name: { type: String },
  },
  description:
    'svy registry 查看所有源\nsvy registry <name> 切换对应源\nsvy registry taobao=https://registry.npmmirror.com/ 添加或修改源',
  action({ name }) {
    isUndefined(name) ? list() : setRegistry(name);
    console.log('\n');
  },
});
