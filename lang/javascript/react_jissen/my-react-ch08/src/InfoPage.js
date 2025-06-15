import { Link } from 'react-router-dom';

export default function AboutPage() {
  return (
      <>
        <p>Infoページです。</p>
        <lu>
            <li><Link to="..">トップ</Link></li>
            <li><Link to=".." relative="path">トップ(relative=path)</Link></li>
            <li><Link to=".." reloadDocument>トップ(reloadDocument)</Link></li>
        </lu>
      </>
  );
}
