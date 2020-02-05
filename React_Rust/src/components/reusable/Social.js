import React from "react";

const YouTube = () => (<a
	href="https://www.youtube.com/channel/UCt_jsJOe91EVjd58kHpgTfw"
	className="no-text-decoration hover-more-opacity transition-half"
	target="_blank"
	rel="noopener noreferrer"
	title="Steadylearner YouTube"
>
	<i className="fab fa-youtube red" />
</a>);

const Twitter = () => (<a
	href="https://twitter.com/steadylearner_p"
	className="no-text-decoration hover transition-half"
	target="_blank"
	rel="noopener noreferrer"
	title="Steadylearner YouTube"
>
	<i className="fa fa-twitter blue margin-left-one" />
</a>);

const GitHub = () => <a
	href="https://github.com/steadylearner"
	className=" no-text-decoration white flex center font-normal hover transition-half "
	target="_blank"
	rel="noopener noreferrer"
	title="Steadylearner GitHub"
>
	<span className="white font-normal fab fa-github">
		<span className="margin-left-half red-white">
            Steadylearner
		</span>
	</span>
</a>;

export {
	YouTube,
	Twitter,
	GitHub,
};
