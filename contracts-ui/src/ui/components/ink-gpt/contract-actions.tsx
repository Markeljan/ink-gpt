
import axios from 'axios';

export const ContractActions = ({ sourceCode }: { sourceCode: string }) => {


    async function compile() {
        try {
            const res = await axios.post('http://127.0.0.1:8000/api/compile', { source: sourceCode }, {
                headers: {
                    'Content-Type': 'application/json'
                }
            });
            console.log(res);
        } catch (error) {
            console.error(error);
        }
    }

function deploy() {
    // call deploy endpoint
}

return (
    <div className="flex items-center space-x-1">
        <button onClick={() => compile()}>
            Compile
        </button>
        <button onClick={() => deploy()}>
            Deploy
        </button>
    </div>
);
};
