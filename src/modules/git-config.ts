import { execSync } from 'child_process';
import { checkConfig, createAction, isUndefined } from '../common/utils';

const actionMap = {
  get: () => {
    const getConfig = (scope?: string) => {
      scope = scope === 'global' ? '--global' : '';
      const username = execSync(`git config ${scope} --get user.name`).toString('utf-8').trim();
      const email = execSync(`git config ${scope} --get user.email`).toString('utf-8').trim();
      console.log(`user.name:\t${username}\nuser.email:\t${email}`);
    };

    console.log('全局配置:');
    getConfig('global');
    console.log(`========================================`);
    console.log('当前仓库配置:');
    getConfig();
  },

  preset: () => {
    const gitConfigList = checkConfig().gitConfigList;
    Object.keys(gitConfigList).forEach((key) => {
      const [username, email] = gitConfigList[key];
      console.log(`${key}\t${username}\n\t\t${email}`);
    });
  },

  use: (key?: string) => {},
};

export const gitConfig = createAction({
  name: 'git-config',
  args: { name: { type: String } },
  description: 'svy git-config 查看git仓库配置的用户名和邮箱',
  action({ name }) {
    name = isUndefined(name) ? 'get' : name;
    actionMap[name as keyof typeof actionMap]();
    console.log('\n');
  },
});