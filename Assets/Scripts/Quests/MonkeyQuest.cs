using UnityEngine;

/// <summary>
/// The "find the lost monkey baby" quest.
/// NPCs give indirect hints about the monkey's location.
/// </summary>
public class MonkeyQuest : MonoBehaviour
{
    [Header("Quest State")]
    public bool isActive;
    public bool isCompleted;

    [Header("Hints by Difficulty")]
    [TextArea] public string hintKindergarten = "Das Äffchen ist bei den großen grauen Tieren!";
    [TextArea] public string hintGrade1 = "Suche bei dem Tier, das einen Rüssel hat.";
    [TextArea] public string hintGrade2 = "Das Äffchen versteckt sich in der Nähe vom Wasser, wo die größten Tiere im Zoo leben.";
    [TextArea] public string hintGrade3 = "Ein Besucher hat das Äffchen zuletzt in der Nähe des Geheges gesehen, dessen Bewohner aus Afrika stammen und sehr schwer sind.";

    public string GetHint()
    {
        var difficulty = GameManager.Instance.GetDifficulty();
        return difficulty switch
        {
            GameManager.Difficulty.Kindergarten => hintKindergarten,
            GameManager.Difficulty.Grade1 => hintGrade1,
            GameManager.Difficulty.Grade2 => hintGrade2,
            GameManager.Difficulty.Grade3 => hintGrade3,
            _ => hintKindergarten
        };
    }

    public void Complete()
    {
        isCompleted = true;
        GameManager.Instance.AddScore(50);
        Debug.Log("Das Affenbaby wurde gefunden!");
    }
}
