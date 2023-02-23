use axum::extract::Path;
use axum::response::IntoResponse;
use serde_json::json;
use model::PostDetail;
use axum::Json;
use crate::errors::Result;


pub async fn post(Path(id): Path<i64>) -> Result<impl IntoResponse> {
    let response = PostDetail {
        id,
        title: "改造我们的学习".to_string(),
        content: "　　这是毛泽东在延安干部会上所作的报告。这篇报告和《整顿党的作风》、《反对党八股》，是毛泽东关于整风运动的基本著作。在这些文章里，毛泽东进一步地从思想问题上总结了过去中国共产党内路线的分歧，分析了广泛存在于党内的非马克思列宁主义思想作风，主要是主观主义的倾向，宗派主义的倾向，和作为这两种倾向的表现形式的党八股。毛泽东号召开展全党范围的马克思列宁主义的教育运动，即按照马克思列宁主义的思想原则整顿作风的运动。毛泽东的这个号召，很快地在中国共产党内和党外引起了怎样以从实际出发的观点而不是以教条主义的观点来对待马克思列宁主义原理，怎样使马克思列宁主义的基本原理和中国革命的实际相结合，以及怎样对待一九三一年初至一九三四年底这段时期党内两条路线的斗争这样一些重大问题的大讨论，巩固了马克思列宁主义思想在党内外的阵地，使广大干部在思想上大大地提高了一步，使中国共产党达到了空前的团结。".to_string(),
        create_time: -650720028,
    };

    Ok(Json(json!(response)))
}
