interface ButtonProps {
	label: string;
	onClick: () => void;
	disabled?: boolean;
}

// simple button with no deps
export function Button({ label, onClick, disabled = false }: ButtonProps) {
	return (
		<button disabled={disabled} onClick={onClick}>
			{label}
		</button>
	);
}
