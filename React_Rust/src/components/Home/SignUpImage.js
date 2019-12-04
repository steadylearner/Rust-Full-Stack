import React, { useState } from "react";
import { useTransition, animated } from "react-spring";

export default function SignUpImage()	{
	const [show, setShow] = useState(false);

	const transitions = useTransition(show, null, {
		from: { opacity: 0 },
		enter: { opacity: 1 },
		leave: { opacity: 0 },
	});

	const SteadylearnerLogo = () => transitions.map(({ item, key, props }) => item && <animated.span
		className="bold font-two-and-a-half white"
		style={props}
		key={key}
	>
		©ode
	</animated.span>);

	return (
		<section
			className="
                sign-up__image
                width-percent
                height-percent
                background-percent
				hover cursor-pointer transition-half
				flex center
			"
			onClick={() => setShow(!show)}
		>
			<SteadylearnerLogo />
		</section>
	);
}
