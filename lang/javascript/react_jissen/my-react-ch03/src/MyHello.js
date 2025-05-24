import PropTypes from 'prop-types';

function MyHello(props) {
    return (
        <div>こんにちは、{props.myName}さん</div>
    );
}

MyHello.propTypes = {
    myName: PropTypes.string.isRequired
};

export default MyHello;