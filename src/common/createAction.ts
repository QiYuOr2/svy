type ArgType<T> = () => T;
type ArgsOptions<T> = {
  type: ArgType<T>;
  required?: boolean;
};
type Args<T> = {
  [K in keyof T]: ArgsOptions<T[K]>;
};

type Options<ArgType> = {
  name: string;
  args: Args<ArgType>;
  description: string;
  action: (args: ArgType) => void;
};

export function createAction<ArgType>(options: Options<ArgType>) {
  const isString = (val: unknown) => typeof val === 'string';
  const isNumber = (val: unknown) => typeof val === 'number';
  const isBoolean = (val: unknown) => typeof val === 'boolean';

  let optionsStrArr = [options.name];
  for (const key in options.args) {
    const argv = options.args[key];

    optionsStrArr.push(argv.required ? `<${key}>` : `[${key}]`);
  }

  return {
    optionsStr: optionsStrArr.join(' '),
    description: options.description,
    action: (...args: any[]) => {
      let params: Record<string, any> = {};
      Object.keys(options.args).forEach((argv, i) => {
        isString(options.args[argv as keyof ArgType].type()) && (params[argv] = String(args[i]));
        isNumber(options.args[argv as keyof ArgType].type()) && (params[argv] = Number(args[i]));
        isBoolean(options.args[argv as keyof ArgType].type()) && (params[argv] = Boolean(args[i]));
      });
      options.action(params as ArgType);
    },
  };
}
