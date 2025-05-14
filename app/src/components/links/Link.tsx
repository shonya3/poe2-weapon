import { RouterLink, RouterLinkProps } from 'vue-router';
import { defineComponent } from 'vue';
import { command } from '../../command';

export type Props =
	| { kind: 'app'; to: RouterLinkProps['to'] }
	| { kind: 'app'; onClick: () => void }
	| { kind: 'browser'; to: string };

export const Link = defineComponent<Props>({
	props: ['kind', 'to', 'onClick'],
	emits: ['click'],
	setup(props, { slots }) {
		const css_class = 'text-sm cf:text-base underline cursor-pointer text-blue-600 dark:text-blue-500';
		return () => {
			switch (props.kind) {
				case 'browser': {
					return (
						<a
							onClick={e => {
								e.preventDefault();
								command('open_browser', { url: props.to });
							}}
							class={css_class}
							href={props.to}
						>
							{slots.default?.()}
						</a>
					);
				}

				case 'app': {
					if ('to' in props && props.to) {
						return (
							<RouterLink class={css_class} to={props.to}>
								{slots.default?.()}
							</RouterLink>
						);
					}

					if ('onClick' in props && props.onClick) {
						return (
							<button class={css_class} onClick={props.onClick}>
								{slots.default?.()}
							</button>
						);
					}

					return null;
				}

				default: {
					throw new Error(`Not supported kind`);
				}
			}
		};
	},
});
